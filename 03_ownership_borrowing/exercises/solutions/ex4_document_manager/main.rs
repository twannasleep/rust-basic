use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, RwLock};

// Document structure with content and metadata
#[derive(Clone)]
struct Document {
    title: String,
    content: String,
    version: u32,
}

impl Document {
    fn new(title: String, content: String) -> Self {
        Document {
            title,
            content,
            version: 1,
        }
    }

    fn update_content(&mut self, new_content: String) {
        self.content = new_content;
        self.version += 1;
    }
}

// Document collection that manages concurrent access
struct DocumentCollection {
    documents: Arc<RwLock<HashMap<String, Arc<RwLock<Document>>>>>,
}

impl DocumentCollection {
    fn new() -> Self {
        DocumentCollection {
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Add a new document
    fn add_document(&self, doc: Document) -> io::Result<()> {
        let mut docs = self.documents.write().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Failed to acquire write lock")
        })?;

        if docs.contains_key(&doc.title) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Document already exists",
            ));
        }

        docs.insert(doc.title.clone(), Arc::new(RwLock::new(doc)));
        Ok(())
    }

    // Get a read-only reference to a document
    fn read_document(&self, title: &str) -> io::Result<Arc<RwLock<Document>>> {
        let docs = self.documents.read().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Failed to acquire read lock")
        })?;

        docs.get(title)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Document not found"))
    }

    // Move a document between collections
    fn move_document(
        &self,
        other: &DocumentCollection,
        title: &str,
    ) -> io::Result<()> {
        let mut source_docs = self.documents.write().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Failed to acquire write lock on source")
        })?;

        let doc = source_docs
            .remove(title)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Document not found"))?;

        let mut dest_docs = other.documents.write().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Failed to acquire write lock on destination")
        })?;

        dest_docs.insert(title.to_string(), doc);
        Ok(())
    }

    // List all document titles
    fn list_documents(&self) -> io::Result<Vec<String>> {
        let docs = self.documents.read().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Failed to acquire read lock")
        })?;

        Ok(docs.keys().cloned().collect())
    }
}

// Track active readers and writers
struct AccessTracker {
    readers: Mutex<HashMap<String, u32>>,
}

impl AccessTracker {
    fn new() -> Self {
        AccessTracker {
            readers: Mutex::new(HashMap::new()),
        }
    }

    fn add_reader(&self, title: &str) {
        let mut readers = self.readers.lock().unwrap();
        *readers.entry(title.to_string()).or_insert(0) += 1;
    }

    fn remove_reader(&self, title: &str) {
        let mut readers = self.readers.lock().unwrap();
        if let Some(count) = readers.get_mut(title) {
            *count -= 1;
            if *count == 0 {
                readers.remove(title);
            }
        }
    }

    fn get_reader_count(&self, title: &str) -> u32 {
        let readers = self.readers.lock().unwrap();
        *readers.get(title).unwrap_or(&0)
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
    let collection_a = DocumentCollection::new();
    let collection_b = DocumentCollection::new();
    let access_tracker = Arc::new(AccessTracker::new());

    println!("Document Manager");
    println!("---------------");

    loop {
        println!("\nOptions:");
        println!("1. Create document");
        println!("2. Read document");
        println!("3. Edit document");
        println!("4. Move document");
        println!("5. List documents");
        println!("6. Show document stats");
        println!("7. Quit");

        let choice = get_user_input("\nSelect option (1-7): ");

        match choice.as_str() {
            "1" => {
                let title = get_user_input("Enter document title: ");
                println!("Enter content (empty line to finish):");
                let mut content = String::new();
                loop {
                    let line = get_user_input("");
                    if line.is_empty() {
                        break;
                    }
                    content.push_str(&line);
                    content.push('\n');
                }

                let doc = Document::new(title.clone(), content);
                match collection_a.add_document(doc) {
                    Ok(_) => println!("Document created successfully!"),
                    Err(e) => println!("Error creating document: {}", e),
                }
            }
            "2" => {
                let title = get_user_input("Enter document title: ");
                match collection_a.read_document(&title) {
                    Ok(doc) => {
                        if let Ok(doc) = doc.read() {
                            println!("\nTitle: {}", doc.title);
                            println!("Version: {}", doc.version);
                            println!("Content:\n{}", doc.content);
                            access_tracker.add_reader(&title);
                        }
                    }
                    Err(e) => println!("Error reading document: {}", e),
                }
            }
            "3" => {
                let title = get_user_input("Enter document title: ");
                if access_tracker.get_reader_count(&title) > 0 {
                    println!("Cannot edit: document is being read");
                    continue;
                }

                match collection_a.read_document(&title) {
                    Ok(doc) => {
                        println!("Enter new content (empty line to finish):");
                        let mut content = String::new();
                        loop {
                            let line = get_user_input("");
                            if line.is_empty() {
                                break;
                            }
                            content.push_str(&line);
                            content.push('\n');
                        }

                        if let Ok(mut doc) = doc.write() {
                            doc.update_content(content);
                            println!("Document updated successfully!");
                        }
                    }
                    Err(e) => println!("Error accessing document: {}", e),
                }
            }
            "4" => {
                let title = get_user_input("Enter document title: ");
                if access_tracker.get_reader_count(&title) > 0 {
                    println!("Cannot move: document is being read");
                    continue;
                }

                match collection_a.move_document(&collection_b, &title) {
                    Ok(_) => println!("Document moved successfully!"),
                    Err(e) => println!("Error moving document: {}", e),
                }
            }
            "5" => {
                println!("\nCollection A:");
                match collection_a.list_documents() {
                    Ok(docs) => {
                        for doc in docs {
                            println!("- {}", doc);
                        }
                    }
                    Err(e) => println!("Error listing documents: {}", e),
                }

                println!("\nCollection B:");
                match collection_b.list_documents() {
                    Ok(docs) => {
                        for doc in docs {
                            println!("- {}", doc);
                        }
                    }
                    Err(e) => println!("Error listing documents: {}", e),
                }
            }
            "6" => {
                println!("\nActive readers:");
                match collection_a.list_documents() {
                    Ok(docs) => {
                        for title in docs {
                            let count = access_tracker.get_reader_count(&title);
                            if count > 0 {
                                println!("- {}: {} reader(s)", title, count);
                            }
                        }
                    }
                    Err(e) => println!("Error getting document stats: {}", e),
                }
            }
            "7" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please select 1-7."),
        }
    }
} 