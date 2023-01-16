extern crate cursive;

struct XTerminal {
    mut cursive_term: cursive::CursiveRunnable;
    
    m_memory: &Vec<u8>;
    mut width: u16;
    mut height: u16;
    mut m_memaddr: u64;
}

impl XTerminal {
    fn Initialise(m_memory: &Vec<u8>, mut width: u16, mut height: u16, mut m_memaddr: u64) {
        self.m_memory = m_memory;
        self.width = width;
        self.height = height;
        self.m_memaddr = m_memaddr;

        self.cursive_term = cursive::default();
        self.cursive_term.run();
    }
}
