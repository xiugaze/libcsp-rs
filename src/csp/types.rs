#[repr(C)]
pub struct CspPacket {
    length: u16,
    // id: CspID,
    // next: &CspPacket
    header: [u8; 8],
    data: Vec<u8>,
}
