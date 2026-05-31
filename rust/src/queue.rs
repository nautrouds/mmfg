use crate::layout;
use crate::shm::SharedMemory;

pub struct Request {
    pub cmd: u32,
    pub slot_id: u32,
}

pub struct RequestQueue {
    node_id: usize,
    shm: SharedMemory,
}

impl RequestQueue {
    pub fn new(node_id: usize, shm: SharedMemory) -> Self {
        Self { node_id, shm }
    }

    pub fn poll(&self) -> Option<Request> {
        let base_offset = layout::get_node_req_queue_offset(self.node_id);
        let head = self.shm.read_u32(base_offset + layout::Q_OFF_HEAD);
        let tail = self.shm.read_u32(base_offset + layout::Q_OFF_TAIL);

        if head == tail {
            return None;
        }

        let entry_offset = base_offset + layout::Q_OFF_ENTRIES + ((head as usize % layout::QUEUE_SIZE) * 8);
        let entry = self.shm.read_u32(entry_offset);

        // Check if ready bit is set
        if (entry & layout::Q_READY_BIT) == 0 {
            return None;
        }

        let cmd = entry & !layout::Q_READY_BIT;
        let slot_id = self.shm.read_u32(entry_offset + 4);

        // Update head
        self.shm.write_u32(base_offset + layout::Q_OFF_HEAD, head.wrapping_add(1));

        Some(Request { cmd, slot_id })
    }
}
