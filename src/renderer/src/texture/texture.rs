
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct TextureUsage: u32 {
        /// Allows a texture to be the source
        const COPY_SRC = 1;
        /// Allows a texture to be the destination
        const COPY_DST = 2;
        /// Allows a texture to be a sampled texture in a bind group
        const SAMPLED = 4;
        /// Allows a texture to be a storage texture in a bind group
        const STORAGE = 8;
        /// Allows a texture to be an output attachment of a render pass
        const OUTPUT_ATTACHMENT = 16;
        /// None of the bits set
        const NONE = 0;
    }
}
