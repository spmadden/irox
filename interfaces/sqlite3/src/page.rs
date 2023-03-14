use std::io::{Read, Seek};

use crate::{error::Error, header::Header};
use irox_tools::bits::Bits;

#[derive(Debug)]
pub struct PageHeader {
    /// the integer identifier of this page
    page_id: u32,

    /// the length in bytes of this page
    page_size: u32,

    /// the offset in bytes from the start of the file
    page_offset: u64,

    /// The one-byte flag at offset 0 indicating the b-tree page type.
    page_type: u8,

    /// The two-byte integer at offset 1 gives the start of the first
    /// freeblock on the page, or is zero if there are no freeblocks.
    first_freeblock: u16,

    /// The two-byte integer at offset 3 gives the number of cells on the page.
    num_cells: u16,

    /// The two-byte integer at offset 5 designates the start of the cell
    /// content area. A zero value for this integer is interpreted as 65536.
    first_cell: u16,

    /// The one-byte integer at offset 7 gives the number of fragmented free
    /// bytes within the cell content area.
    num_fragmented_free_bytes: u8,

    /// The four-byte page number at offset 8 is the right-most pointer. This
    /// value appears in the header of interior b-tree pages only and is
    /// omitted from all other pages.
    rightmost_pointer: u32,
}

#[derive(Debug)]
pub struct InteriorIndex {
    page_header: PageHeader,
    rightmost_pointer: u32,
}

#[derive(Debug)]
pub struct InteriorTable {
    page_header: PageHeader,
    rightmost_pointer: u32,
}

#[derive(Debug)]
pub struct LeafIndex {
    page_header: PageHeader,
}

#[derive(Debug)]
pub struct LeafTable {
    page_header: PageHeader,
}

pub struct DataExtension {
    data: [u8],
}

#[derive(Debug)]
pub enum PageType {
    Unknown,
    InteriorIndexBTree(InteriorIndex),
    InteriorTableBTree(InteriorTable),
    LeafIndexBTree(LeafIndex),
    LeafTableBTree(LeafTable),
}

pub fn read_page<T: Read + Seek>(
    buffer: &mut T,
    page_id: u32,
    db_header: &Header,
) -> Result<PageType, Error> {
    let page_count = db_header.page_count;
    let page_size = db_header.page_size as u32;
    if page_id >= page_count {
        return Err(Error::new(
            format!("Page number {page_id} >= {page_count}").as_str(),
        ));
    }

    let mut page_offset = page_size as u64 * page_id as u64;
    if page_id == 0 {
        page_offset = 100; // from the 100 byte header
    }

    buffer.seek(std::io::SeekFrom::Start(page_offset))?;

    // read header
    let page_type: u8 = buffer.read_u8()?;
    let first_freeblock = buffer.read_be_u16()?;
    let num_cells = buffer.read_be_u16()?;
    let first_cell = buffer.read_be_u16()?;
    let num_fragmented_free_bytes = buffer.read_u8()?;

    let mut rightmost_pointer = 0;
    if page_type == INTERIOR_INDEX || page_type == INTERIOR_TABLE {
        rightmost_pointer = buffer.read_be_u32()?;
    }

    let page_header = PageHeader {
        page_id,
        page_size,
        page_offset,
        page_type,
        first_freeblock,
        num_cells,
        first_cell,
        num_fragmented_free_bytes,
        rightmost_pointer,
    };

    match page_type {
        INTERIOR_INDEX => Ok(PageType::InteriorIndexBTree(InteriorIndex {
            page_header,
            rightmost_pointer,
        })),
        INTERIOR_TABLE => Ok(PageType::InteriorTableBTree(InteriorTable {
            page_header,
            rightmost_pointer,
        })),
        LEAF_INDEX => Ok(PageType::LeafIndexBTree(LeafIndex { page_header })),
        LEAF_TABLE => Ok(PageType::LeafTableBTree(LeafTable { page_header })),
        _ => Err(Error::new("Invalid page type")),
    }
}

pub const INTERIOR_INDEX: u8 = 0x02;
pub const INTERIOR_TABLE: u8 = 0x05;
pub const LEAF_INDEX: u8 = 0x0A;
pub const LEAF_TABLE: u8 = 0x0D;
