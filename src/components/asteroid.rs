


pub struct Asteroid {
    root: AsteroidChunk,
}

pub struct  AsteroidChunk {
        children: Option<[Box<AsteroidChunk>;4]>,
}
