use std::io::{Read, Seek};

use crate::{error::Error, header::Header};

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

pub struct InteriorIndex {
    page_header: PageHeader,
    rightmost_pointer: u32,
}

pub struct InteriorTable {
    page_header: PageHeader,
    rightmost_pointer: u32,
}

pub struct LeafIndex {
    page_header: PageHeader,
}

pub struct LeafTable {
    page_header: PageHeader,
}

pub enum PageType {
    Unknown,
    InteriorIndexBTree(InteriorIndex),
    InteriorTableBTree(InteriorTable),
    LeafIndexBTree(LeafIndex),
    LeafTableBTree(LeafTable),
}

pub fn read_page<T: Read + Seek>(
    buffer: &mut T,
    page_num: u32,
    db_header: &Header,
) -> Result<PageType, Error> {
    let page_count = db_header.page_count;
    if page_num >= page_count {
        return Err(Error::new(format!("Page number {page_num} >= {page_count}").as_str()));
    }

    let page_offset = db_header.page_size as u32 * page_num;
    

    todo!()
}

pub const INTERIOR_INDEX: u8 = 0x02;
pub const INTERIOR_TABLE: u8 = 0x05;
pub const LEAF_INDEX: u8 = 0x0A;
pub const LEAF_TABLE: u8 = 0x0D;
