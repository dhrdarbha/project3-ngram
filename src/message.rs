/// A request from the client to the server
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Add the document `doc` to the archive
    Publish { doc: String },
    /// Search for the word `word` in the archive
    Search { word: String },
    /// Retrieve the document with the index `id` from the archive
    Retrieve { id: usize },
}
impl Request {
    // TODO:
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Request::Publish { doc } => {
                bytes.push(0);
                let doc_bytes = doc.as_bytes();
                bytes.extend(&(doc_bytes.len() as u64).to_be_bytes());
                bytes.extend(doc_bytes);
            }
            Request::Search { word } => {
                bytes.push(1);
                let word_bytes = word.as_bytes();
                bytes.extend(&(word_bytes.len() as u64).to_be_bytes());
                bytes.extend(word_bytes);
            }
            Request::Retrieve { id } => {
                bytes.push(2);
                bytes.extend(&(*id as u64).to_be_bytes());
            }
        }
        bytes
    }
    // TODO:
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut tag = [0u8; 1];
        reader.read_exact(&mut tag).ok()?;

        match tag[0] {
            0 => {
                let mut len_bytes = [0u8; 8];
                reader.read_exact(&mut len_bytes).ok()?;
                let len = u64::from_be_bytes(len_bytes) as usize;

                let mut doc_bytes = vec![0u8; len];
                reader.read_exact(&mut doc_bytes).ok()?;
                let doc = String::from_utf8(doc_bytes).ok()?;

                Some(Request::Publish { doc })
            }
            1 => {
                let mut len_bytes = [0u8; 8];
                reader.read_exact(&mut len_bytes).ok()?;
                let len = u64::from_be_bytes(len_bytes) as usize;

                let mut word_bytes = vec![0u8; len];
                reader.read_exact(&mut word_bytes).ok()?;
                let word = String::from_utf8(word_bytes).ok()?;

                Some(Request::Search { word })
            }
            2 => {
                let mut id_bytes = [0u8; 8];
                reader.read_exact(&mut id_bytes).ok()?;
                let id = u64::from_be_bytes(id_bytes) as usize;

                Some(Request::Retrieve { id })
            }
            _ => None,
        }
    }
}

/// A response from the server to the client
#[derive(Debug, PartialEq)]
pub enum Response {
    /// The document was successfully added to the archive with the given index
    PublishSuccess(usize),
    /// The search for the word was successful, and the indices of the documents containing the
    /// word are returned
    SearchSuccess(Vec<usize>),
    /// The retrieval of the document was successful, and the document is returned
    RetrieveSuccess(String),
    /// The request failed
    Failure,
}
impl Response {
    // TODO:
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Response::PublishSuccess(id) => {
                bytes.push(0);
                bytes.extend(&(*id as u64).to_be_bytes());
            }
            Response::SearchSuccess(ids) => {
                bytes.push(1);
                bytes.extend(&(ids.len() as u64).to_be_bytes());
                for id in ids {
                    bytes.extend(&(*id as u64).to_be_bytes());
                }
            }
            Response::RetrieveSuccess(doc) => {
                bytes.push(2);
                let doc_bytes = doc.as_bytes();
                bytes.extend(&(doc_bytes.len() as u64).to_be_bytes());
                bytes.extend(doc_bytes);
            }
            Response::Failure => {
                bytes.push(3);
            }
        }
        bytes
    }
    // TODO:
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut tag = [0u8; 1];
        reader.read_exact(&mut tag).ok()?;

        match tag[0] {
            0 => {
                let mut id_bytes = [0u8; 8];
                reader.read_exact(&mut id_bytes).ok()?;
                let id = u64::from_be_bytes(id_bytes) as usize;
                Some(Response::PublishSuccess(id))
            }
            1 => {
                let mut count_bytes = [0u8; 8];
                reader.read_exact(&mut count_bytes).ok()?;
                let count = u64::from_be_bytes(count_bytes) as usize;

                let mut ids = Vec::with_capacity(count);
                for _ in 0..count {
                    let mut id_bytes = [0u8; 8];
                    reader.read_exact(&mut id_bytes).ok()?;
                    let id = u64::from_be_bytes(id_bytes) as usize;
                    ids.push(id);
                }
                Some(Response::SearchSuccess(ids))
            }
            2 => {
                let mut len_bytes = [0u8; 8];
                reader.read_exact(&mut len_bytes).ok()?;
                let len = u64::from_be_bytes(len_bytes) as usize;

                let mut doc_bytes = vec![0u8; len];
                reader.read_exact(&mut doc_bytes).ok()?;
                let doc = String::from_utf8(doc_bytes).ok()?;

                Some(Response::RetrieveSuccess(doc))
            }
            3 => Some(Response::Failure),
            _ => None,
        }
    }
}
