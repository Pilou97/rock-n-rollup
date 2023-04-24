use crate::core::{Runtime, PREIMAGE_HASH_SIZE};

pub struct PreimageHash {
    inner: [u8; PREIMAGE_HASH_SIZE],
}

impl TryFrom<&str> for PreimageHash {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let decoded = hex::decode(value).map_err(|_| ())?;
        let inner = decoded.try_into().map_err(|_| ())?;
        Ok(Self { inner })
    }
}

impl AsRef<[u8; PREIMAGE_HASH_SIZE]> for PreimageHash {
    fn as_ref(&self) -> &[u8; PREIMAGE_HASH_SIZE] {
        &self.inner
    }
}

impl<'a> TryFrom<&'a [u8]> for PreimageHash {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let inner = value.try_into().map_err(|_| ())?;
        Ok(Self { inner })
    }
}

impl PreimageHash {
    fn new(inner: [u8; PREIMAGE_HASH_SIZE]) -> Self {
        Self { inner }
    }
}

/// Borrowing version of [V0ContentPage].
#[derive(Debug)]
pub struct V0SliceContentPage<'a> {
    inner: &'a [u8],
}

/// Borrowing version of [V0HashPage].
#[derive(Debug)]
pub struct V0SliceHashPage<'a> {
    // Guaranteed to be a multiple of PREIMAGE_HASH_SIZE
    inner: &'a [u8],
}

/// A Dac [Page] that borrows the underlying buffer.
///
/// Can be used in `no_std` & `alloc`-free environments.
#[derive(Debug)]
pub enum SlicePage<'a> {
    /// Contents of borrowed bytes.
    V0ContentPage(V0SliceContentPage<'a>),
    /// Contents of borrowed hashes.
    V0HashPage(V0SliceHashPage<'a>),
}

/// Errors that may occur when dealing with [SlicePage].
#[derive(Debug)]
pub enum SlicePageError {
    /// Unknown page tag.
    InvalidTag(Option<u8>),
    /// Invalid size prefix.
    InvalidSizePrefix,
}

/// Maximum size of dac pages is 4Kb.
pub const MAX_PAGE_SIZE: usize = 4096;

/// Tag size to distinguish hash/contents pages.
pub(crate) const PAGE_TAG_SIZE: usize = 1;

/// Prefix of 4-bytes to define how large contents/hash page is.
pub(crate) const PAGE_SIZE_PREFIX_SIZE: usize = 4;

/// Maximum content/hashes size that can fit in a page.
pub(crate) const MAX_USABLE_PAGE_SIZE: usize =
    MAX_PAGE_SIZE - (PAGE_TAG_SIZE + PAGE_SIZE_PREFIX_SIZE);

impl<'a> V0SliceContentPage<'a> {
    /// Maximum size of content in each page.
    pub const MAX_CONTENT_SIZE: usize = MAX_USABLE_PAGE_SIZE;

    // Assumes magic byte has been discarded
    fn parse(slice: &'a [u8]) -> Result<Self, SlicePageError> {
        if slice.len() < 4 {
            return Err(SlicePageError::InvalidSizePrefix);
        }

        let size = u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]) as usize;

        let end_offset = 4 + size;

        if slice.len() < end_offset {
            return Err(SlicePageError::InvalidSizePrefix);
        }

        Ok(Self {
            inner: &slice[4..end_offset],
        })
    }
}

impl<'a> V0SliceHashPage<'a> {
    /// Maximum number of hashes able to fit into a hash page.
    pub const MAX_HASHES_PER_PAGE: usize = MAX_USABLE_PAGE_SIZE / PREIMAGE_HASH_SIZE;

    /// Returns an iterator over the preimage hashes contained within.
    pub fn hashes(&self) -> impl Iterator<Item = &'a [u8; PREIMAGE_HASH_SIZE]> {
        // there is a nightly(only) API called `as_chunks` that would return
        // `(&[[u8; PREIMAPREIMAGE_HASH_SIZE]], &[u8])` that we could use in
        // future
        self.inner
            .chunks_exact(PREIMAGE_HASH_SIZE)
            .map(|chunk| chunk.try_into().expect("Guaranteed to be exact."))
    }

    // Assumes magic byte has been discarded
    fn parse(slice: &'a [u8]) -> Result<Self, SlicePageError> {
        if slice.len() < 4 {
            return Err(SlicePageError::InvalidSizePrefix);
        }

        let size = u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]) as usize;

        let end_offset = 4 + size; // for prefix bytes

        if slice.len() < end_offset || size % PREIMAGE_HASH_SIZE != 0 {
            return Err(SlicePageError::InvalidSizePrefix);
        }

        Ok(Self {
            inner: &slice[4..end_offset],
        })
    }
}

impl<'a> TryFrom<&'a [u8]> for SlicePage<'a> {
    type Error = SlicePageError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        match value {
            [0, rest @ ..] => Ok(SlicePage::V0ContentPage(V0SliceContentPage::parse(rest)?)),
            [1, rest @ ..] => Ok(SlicePage::V0HashPage(V0SliceHashPage::parse(rest)?)),
            _ => Err(SlicePageError::InvalidTag(value.first().cloned())),
        }
    }
}

/// Recursively traverses a Merkle Tree of hashes up to `max_dac_levels` depth where each hash
/// corresponds to a preimage that can be revealed via [Runtime::reveal_preimage]. The closure
/// `save_content` is applied on each content page found.
///
/// N.B `max_dac_levels`, should probably be kept under 4 (4 gives 7.9GB of data approximately)
pub fn reveal_loop<Host: Runtime>(
    host: &mut Host,
    level: usize,
    hash: &PreimageHash,
    max_dac_levels: usize,
    acc: &mut Vec<Vec<u8>>,
) -> Result<(), ()> {
    if level >= max_dac_levels {
        return Err(());
    }

    let page = host.reveal_preimage(hash.as_ref())?;
    let page = page.as_ref();

    let page = SlicePage::try_from(page).map_err(|_| ())?;

    match page {
        SlicePage::V0HashPage(hashes) => {
            for hash in hashes.hashes() {
                let hash = PreimageHash::new(hash.clone()); // TODO: avoid cloning
                reveal_loop(host, level + 1, &hash, max_dac_levels, acc)?;
            }
            Ok(())
        }
        SlicePage::V0ContentPage(content) => {
            let content = content.inner.to_vec();
            acc.push(content);
            Ok(())
        }
    }
}

pub trait Dac {
    /// Read the data from the DAC and returns you the data as a vector of bytes
    fn read_from_dac(&mut self, hash: &PreimageHash) -> Result<Vec<u8>, ()>;
}

impl<R> Dac for R
where
    R: Runtime,
{
    fn read_from_dac(&mut self, hash: &PreimageHash) -> Result<Vec<u8>, ()> {
        let mut data = Vec::default();
        let () = reveal_loop(self, 0, hash, 3, &mut data)?;
        let data = data.iter().flatten().copied().collect::<Vec<u8>>();
        Ok(data)
    }
}
