use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Graph {
    edges: Arc<HashMap<u64, Arc<String>>>,
}
impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: Arc::new(HashMap::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_thing() {
        let graph = Graph::new();
    }
}
