use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{self, Write};
use rand::Rng;

// Task structure for priority queue example
#[derive(Debug, Eq, PartialEq)]
struct Task {
    priority: u32,
    description: String,
}

// Implement ordering for Task to use in BinaryHeap
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Cache implementation using VecDeque with fixed size
struct LRUCache<K, V> {
    capacity: usize,
    cache: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Clone + Eq + std::hash::Hash, V> LRUCache<K, V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.cache.len() >= self.capacity && !self.cache.contains_key(&key) {
            if let Some(oldest) = self.order.pop_back() {
                self.cache.remove(&oldest);
            }
        }

        if let Some(idx) = self.order.iter().position(|k| k == &key) {
            self.order.remove(idx);
        }

        self.order.push_front(key.clone());
        self.cache.insert(key, value);
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(idx) = self.order.iter().position(|k| k == key) {
            self.order.remove(idx);
            self.order.push_front(key.clone());
        }
        self.cache.get(key)
    }
}

// Graph implementation using HashMap
#[derive(Debug)]
struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.edges
            .entry(from.to_string())
            .or_insert_with(HashSet::new)
            .insert(to.to_string());
    }

    fn get_neighbors(&self, vertex: &str) -> Vec<&String> {
        self.edges
            .get(vertex)
            .map(|neighbors| neighbors.iter().collect())
            .unwrap_or_default()
    }
}

// Ordered dictionary using BTreeMap
struct OrderedDict<K, V> {
    map: BTreeMap<K, V>,
}

impl<K: Ord, V> OrderedDict<K, V> {
    fn new() -> Self {
        OrderedDict {
            map: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Advanced Collections Demo");
    println!("----------------------\n");

    loop {
        println!("\nSelect demo:");
        println!("1. Priority Queue (BinaryHeap)");
        println!("2. LRU Cache (HashMap + VecDeque)");
        println!("3. Graph (HashMap + HashSet)");
        println!("4. Ordered Dictionary (BTreeMap)");
        println!("5. Sorted Set Operations (BTreeSet)");
        println!("6. Quit");

        let choice = get_user_input("\nEnter choice (1-6): ");

        match choice.as_str() {
            "1" => {
                println!("\nPriority Queue Demo:");
                let mut queue = BinaryHeap::new();

                // Add some tasks
                queue.push(Task {
                    priority: 3,
                    description: "High priority task".to_string(),
                });
                queue.push(Task {
                    priority: 1,
                    description: "Low priority task".to_string(),
                });
                queue.push(Task {
                    priority: 2,
                    description: "Medium priority task".to_string(),
                });

                println!("\nProcessing tasks by priority (highest first):");
                while let Some(task) = queue.pop() {
                    println!("Priority {}: {}", task.priority, task.description);
                }
            }
            "2" => {
                println!("\nLRU Cache Demo:");
                let mut cache = LRUCache::new(3);

                // Add some items
                cache.put("key1", "value1");
                cache.put("key2", "value2");
                cache.put("key3", "value3");
                println!("Added 3 items to cache");

                // Access key2 (moves it to front)
                if let Some(value) = cache.get(&"key2") {
                    println!("Retrieved key2: {}", value);
                }

                // Add new item, should evict oldest
                cache.put("key4", "value4");
                println!("Added key4, cache size: {}", cache.cache.len());
                println!("Try to get key1 (should be None): {:?}", cache.get(&"key1"));
            }
            "3" => {
                println!("\nGraph Demo:");
                let mut graph = Graph::new();

                // Add some edges
                graph.add_edge("A", "B");
                graph.add_edge("A", "C");
                graph.add_edge("B", "C");
                graph.add_edge("C", "D");

                println!("\nGraph structure:");
                for (vertex, edges) in &graph.edges {
                    println!("{} -> {:?}", vertex, edges);
                }

                println!("\nNeighbors of A: {:?}", graph.get_neighbors("A"));
                println!("Neighbors of C: {:?}", graph.get_neighbors("C"));
            }
            "4" => {
                println!("\nOrdered Dictionary Demo:");
                let mut dict = OrderedDict::new();

                // Add items in random order
                let mut rng = rand::thread_rng();
                for i in 0..5 {
                    let key = rng.gen_range(0..100);
                    dict.insert(key, format!("Value {}", i));
                }

                println!("\nItems in sorted order:");
                for (key, value) in dict.iter() {
                    println!("Key: {}, Value: {}", key, value);
                }
            }
            "5" => {
                println!("\nSorted Set Operations Demo:");
                let mut set1 = BTreeSet::new();
                let mut set2 = BTreeSet::new();

                // Add elements to sets
                set1.extend([1, 3, 5, 7, 9]);
                set2.extend([1, 2, 5, 8, 9]);

                println!("\nSet 1: {:?}", set1);
                println!("Set 2: {:?}", set2);

                println!("\nUnion: {:?}", set1.union(&set2).collect::<Vec<_>>());
                println!("Intersection: {:?}", set1.intersection(&set2).collect::<Vec<_>>());
                println!("Difference (Set1 - Set2): {:?}", set1.difference(&set2).collect::<Vec<_>>());
                println!("Symmetric Difference: {:?}", 
                    set1.symmetric_difference(&set2).collect::<Vec<_>>());
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please select 1-6."),
        }
    }
}
