use specs::prelude::*;

enum Furniture
{
    Nothing,
    Table,
    Chair
}

impl Furniture
{
    fn to_char(&self) -> char
    {
	use Furniture::*;
	match self
	{
	    Nothing => '·',
	    Table => 'T',
	    Chair => 'L',
	    _ => '#'
	}
    }
}

#[derive(Default)]
struct TileGrid
{
    grid: Vec<Vec<Furniture>>,
    dim: (usize, usize)
}

struct Room
{
    world: World
}

impl Room
{
    fn new(width: usize,
	   height: usize,
	   dispatcher: &mut Dispatcher) -> Self
    {
	let mut world = World::new();
	dispatcher.setup(&mut world);

	let mut grid = Vec::new();
	for _ in 0..width
	{
	    grid.push((0..height).map(|_| Furniture::Nothing).collect::<Vec<_>>());
	}

	world.insert(
	    TileGrid
	    {
		grid: grid,
		dim: (width, height)
	    }
	);
	
	Self
	{
	    world: world
	}
	
    }

    fn put(&mut self,
	   furn: Furniture,
	   x_pos: usize,
	   y_pos: usize)
    {
	if let Some(mut grid) = self.world.get_mut::<TileGrid>()
	{
	    grid.grid[x_pos][y_pos] = furn;
	}
    }
    
}

impl Component for Room
{
    type Storage = DenseVecStorage<Self>;
}

struct Pos(f32, f32);

impl Component for Pos
{
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
struct Player;

impl Component for Player
{
    type Storage = NullStorage<Self>;
}

struct Printable(char);
impl Component for Printable
{
    type Storage = DenseVecStorage<Self>;
}

struct Spatial();


struct PrintSystem;

impl<'a> System<'a> for PrintSystem
{
    type SystemData = (Read<'a, TileGrid>,
		       ReadStorage<'a, Pos>,
		       ReadStorage<'a, Printable>);

    fn run(&mut self, (grid, poses, chrs): Self::SystemData)
    {

	let mut printing_grid = grid.grid.iter()
	    .map(|vec| vec.iter()
		 .map(|furn| furn.to_char()).collect::<Vec<_>>()
	    ).collect::<Vec<_>>();

	for (pos, chr) in (&poses, &chrs).join()
	{
	    let real_x = pos.0;
	    let real_y = pos.1;

	    let rounded_x = real_x.floor() as usize;
	    let rounded_y = real_y.floor() as usize;

	    if rounded_x >= 0 && rounded_x < grid.dim.0
		&& rounded_y >= 0 && rounded_y < grid.dim.1
	    {
		printing_grid[rounded_x][rounded_y] = chr.0;
	    }
	}

	
	for j in 0..grid.dim.1
	{
	    for i in 0..grid.dim.0
	    {
		print!("{}", printing_grid[i][j]);
	    }
	    println!();
	}
    }
}









fn main()
{
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
	.with(PrintSystem, "printing system", &[])
	.build();
    
    let mut micheline = Room::new(10, 8, &mut dispatcher);

    dispatcher.dispatch(& micheline.world);
    micheline.world.create_entity()
	.with(Pos(1.5, 0.))
	.with(Printable('P'));

    
    println!();
    micheline.put(Furniture::Table, 2, 4);
    dispatcher.dispatch(& micheline.world);
    
}
