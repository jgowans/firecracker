use std::sync::atomic::{AtomicUsize};

//use virtio_gen::virtio_balloon::*;

use std::sync::Arc;
use std::io::{self, Write};

use super::{
    ActivateResult, VirtioDevice, Queue
};
use memory_model::{GuestMemory};
use sys_util::EventFd;
use virtio::{TYPE_BALLOON};
use virtio_gen::virtio_balloon::virtio_balloon_config;

const QUEUE_SIZE: u16 = 256;
const QUEUE_SIZES: &[u16] = &[QUEUE_SIZE, QUEUE_SIZE];

pub struct Balloon {
    config_space: virtio_balloon_config,
    avail_features: u64,
}

impl Balloon {
    pub fn new() -> io::Result<Balloon> {
        let config_space = virtio_balloon_config {
            num_pages: 0u32,
            actual: 0u32,
        };
        Ok(Balloon{
            config_space,
            avail_features: 0u64,
        })
    }
}

impl VirtioDevice for Balloon {
    fn device_type(&self) -> u32 {
        warn!("device_type was called.");
        TYPE_BALLOON
    }

    fn queue_max_sizes(&self) -> &[u16] {
        warn!("queue_max_sizes was called.");
        QUEUE_SIZES
    }

    fn ack_features(&mut self, page: u32, value: u32) {
        // I have no idea what I'm doing...
        warn!("ack_featurse got called with page = {}, value = {}", page, value);
    }

    fn features(&self, page: u32) -> u32 {
        warn!("balloon features call for page {}", page);
        match page {
            0 => 0x30000000u32,
            _ => 0u32
        }
    }

    fn read_config(&self, offset: u64, mut data: &mut [u8]) {
        warn!("read_config got called with offset = {:?}, data = {:?}", offset, data);
        if offset > 0 {
            error!("Only zero length offset is supported. Was asked for: {}", offset);
            return;
        }
        // TODO: use an actual virtio_balloon_config struct.
        let to_write: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
        data.write_all(&to_write);
    }

    fn write_config(&mut self, offset: u64, data: &[u8]) {
        warn!("Got a write config for data {:?} at offset {}", data, offset);
    }

    fn activate(
        &mut self,
        _mem: GuestMemory,
        _interrupt_evt: EventFd,
        _status: Arc<AtomicUsize>,
        _queues: Vec<Queue>,
        mut _queue_evts: Vec<EventFd>,
    ) -> ActivateResult {
        warn!("balloon activate was called");
        Ok(())
    }
}