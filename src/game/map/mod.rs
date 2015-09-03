use session::game::Chunk;

trait Actor {

}

struct Map<'a> {
    parent: &'a Chunk,
}
