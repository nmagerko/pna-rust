use crate::{KvsError, KvsRequest, KvsEngine, Result};
use serde_json::{from_str, to_string};
use std::io::{BufRead, Seek, Write};
use std::{collections, env, fs, io, path};

/// The file name of the primary log file
const LOGFILE: &str = "kvs.log";
/// The file name of the temporary log file while compacting
const COMPACTFILE: &str = "compact.log";
/// The size of the log file needed before compaction occurs
const COMPACT_BYTES: u64 = 1024 * 1024;

/// Stores key-value relationships
pub struct KvStore {
    root: path::PathBuf,
    log: fs::File,
    size: u64,
    entries: collections::HashMap<String, u64>,
}

impl KvStore {
    /// Creates a new `KvStore` with an initialization path in the current directory.
    ///
    /// # Errors
    ///
    /// - A `KvsError::IoError` will occur if the current directory cannot be obtained
    /// - For all other errors, see `KvStore::open`
    pub fn new() -> Result<KvStore> {
        let cwd = env::current_dir()?;
        KvStore::open(cwd.as_path())
    }

    /// Creates a new `KvStore` in a given working directory. All logs, indexes, etc will be
    /// read from or created here.
    ///
    /// # Arguments
    ///
    /// `path` - the
    ///
    /// # Errors
    ///
    /// - A `KvsError::BadPathError` will occur if `path` does not exist or is not a directory
    /// - A `KvsError::EngineMismatchError` will occur if `path` is not compatable with this engine
    /// - A `KvsError::IoError` will occur if file operations fail
    /// - A `KvsError::SerdeError` will occur if reading from the logfile fails
    ///
    /// # Example
    ///
    /// ```
    /// let mut kvs = kvs::KvStore::open(std::path::Path::new("/var/db/"));
    /// ```
    pub fn open(path: &path::Path) -> Result<KvStore> {
        let path_str = path.to_str().unwrap().to_owned();
        if !path.is_dir() {
            return Err(KvsError::BadPathError(path_str));
        }

        let root = path.to_path_buf();
        let (mut log, size) = initialize_logfile(&root)?;
        let entries = initialize_entries(&mut log);
        Ok(KvStore {
            root,
            log,
            size,
            entries,
        })
    }

    fn compact(&self) -> Result<()> {
        // TODO
        unimplemented!();

        let mut compactfile = initialize_compactfile(&self.root)?;
        let mut writer = io::BufWriter::new(&mut compactfile);
        let mut offset: usize = 0;

        for (_, pos) in self.entries.iter_mut() {
            self.log.seek(io::SeekFrom::Start(*pos))?;
            let mut reader = io::BufReader::new(&mut self.log);
            let mut line = String::new();
            reader.read_line(&mut line)?;
            *pos = offset as u64;
            offset += line.len();
            writer.write_all(&(line.into_bytes()))?;
        }
        drop(writer);
        drop(compactfile);
        publish_compactfile(&self.root)?;

        let (log, size) = initialize_logfile(&self.root)?;
        self.log = log;
        self.size = size;
        Ok(())
    }
}

impl KvsEngine for KvStore {
    /// Retrieves the value for a given key (if that key is valid)
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value may be associated
    ///
    /// # Errors
    ///
    /// - A `KvsError::UnknownError` will occur for all internal errors
    /// - A `KvsError::IoError` will occur if file operations fail
    /// - A `KvsError::SerdeError` will occur if reading from the logfile fails
    ///
    /// # Example
    ///
    ///```
    /// use kvs::KvsEngine;
    /// match kvs::KvStore::new() {
    ///     Ok(mut kvs) => { kvs.get(String::from("key")); }
    ///     Err(_) => {}
    /// }
    ///```
    fn get(&self, key: String) -> Result<Option<String>> {
        // TODO
        unimplemented!();

        match self.entries.get(&key) {
            Some(offset) => {
                self.log.seek(io::SeekFrom::Start(*offset))?;
                let mut reader = io::BufReader::new(&mut self.log);
                let mut line = String::new();
                reader.read_line(&mut line)?;

                match from_str(&line) {
                    Ok(KvsRequest::Set { value, .. }) => Ok(Some(value)),
                    Err(err) => Err(KvsError::SerdeError(err)),
                    _ => Err(KvsError::UnknownError),
                }
            }
            None => Ok(None),
        }
    }

    /// Sets a value for a given key. If the key is already present, it is overwrriten.
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value is associated
    /// `value` - the value to be associated
    ///
    /// # Errors
    ///
    /// - A `KvsError::IoError` will occur if file operations fail
    /// - A `KvsError::SerdeError` will occur if seralizing content for the logfile fails
    ///
    /// # Example
    ///
    ///```
    /// use kvs::KvsEngine;
    /// match kvs::KvStore::new() {
    ///     Ok(mut kvs) => { kvs.set(String::from("key"), String::from("value")); }
    ///     Err(_) => {}
    /// }
    ///```
    fn set(&self, key: String, value: String) -> Result<()> {
        // TODO
        unimplemented!();

        let cmd = KvsRequest::Set {
            key: key.to_owned(),
            value,
        };
        let serialized = format!("{}\n", to_string(&cmd)?).into_bytes();
        let offset = self.log.seek(io::SeekFrom::End(0))?;
        self.log.write_all(&serialized)?;
        self.entries.insert(key, offset);
        self.size += serialized.len() as u64;
        if self.size > COMPACT_BYTES {
            self.compact()?;
        }
        Ok(())
    }

    /// Removes a key-value relationship. If the key is not present, nothing happens.
    ///
    /// # Arguments
    ///
    /// `key` - the string with which a value may be associated
    ///
    /// # Errors
    ///
    /// - A `KvsError::BadRemovalError` will occur if the requested key was not found
    /// - A `KvsError::IoError` will occur if file operations fail
    /// - A `KvsError::SerdeError` will occur if seralizing content for the logfile fails
    ///
    /// # Example
    ///
    /// ```
    /// use kvs::KvsEngine;
    /// match kvs::KvStore::new() {
    ///     Ok(mut kvs) => { kvs.remove(String::from("key")); }
    ///     Err(_) => {}
    /// }
    /// ```
    fn remove(&self, key: String) -> Result<()> {
        // TODO
        unimplemented!();
        
        match self.entries.get(&key) {
            Some(_) => {
                let cmd = KvsRequest::Remove {
                    key: key.to_owned(),
                };
                let serialized = format!("{}\n", to_string(&cmd)?).into_bytes();
                self.log.seek(io::SeekFrom::End(0))?;
                self.log.write_all(&serialized)?;
                self.entries.remove(&key);
                self.size += serialized.len() as u64;
                if self.size > COMPACT_BYTES {
                    self.compact()?;
                }
                Ok(())
            }
            None => Err(KvsError::BadRemovalError),
        }
    }
}

impl Clone for KvStore {
    fn clone(&self) -> KvStore {
        unimplemented!();
    }
}

fn initialize_logfile(root: &path::PathBuf) -> std::result::Result<(fs::File, u64), io::Error> {
    let log_path = root.join(LOGFILE);
    let log = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open(&log_path)?;
    let log_size = fs::metadata(log_path)?.len();
    Ok((log, log_size))
}

fn initialize_compactfile(root: &path::PathBuf) -> std::result::Result<fs::File, io::Error> {
    let compact_path = root.join(COMPACTFILE);
    fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(compact_path)
}

fn publish_compactfile(root: &path::PathBuf) -> std::result::Result<(), io::Error> {
    let log_path = root.join(LOGFILE);
    let compact_path = root.join(COMPACTFILE);
    fs::copy(&compact_path, log_path)?;
    fs::remove_file(compact_path)?;
    Ok(())
}

fn initialize_entries(log: &mut fs::File) -> collections::HashMap<String, u64> {
    let mut entries = collections::HashMap::new();
    let reader = io::BufReader::new(log);
    let mut offset: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        match from_str(&line) {
            Ok(KvsRequest::Set { key, .. }) => {
                entries.insert(key, offset as u64);
            }
            Ok(KvsRequest::Remove { key, .. }) => {
                entries.remove(&key);
            }
            _ => {}
        };
        offset += line.len() + 1;
    }
    entries
}
