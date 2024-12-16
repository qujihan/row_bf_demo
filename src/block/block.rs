struct BlockMeta {
    curr_line: u64,
}

// Block需要缓存从文件读取的数据
// 读一行缓存一行
struct Block{
    meta: BlockMeta,
}
