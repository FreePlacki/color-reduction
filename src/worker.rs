use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread,
};

use crate::{
    kmeans_reducer::KMeansReducer, popularity_reducer::PopularityReducer, reducer::Reducer,
    uncert_reducer::UncertReducer,
};

pub struct ComputeRequest {
    pub img: Vec<u8>,
    pub n_colors: usize,
    pub width: usize,
    pub height: usize,
    pub uncert_reducer: Option<UncertReducer>,
    pub popula_reducer: Option<PopularityReducer>,
    pub kmeans_reducer: Option<KMeansReducer>,
}

pub struct ComputeResult {
    pub uncert: Option<Vec<u8>>,
    pub popula: Option<Vec<u8>>,
    pub kmeans: Option<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

pub struct ComputeWorker {
    pub tx: Sender<ComputeRequest>,
    pub rx: Receiver<ComputeResult>,
}

impl ComputeWorker {
    pub fn spawn() -> Self {
        let (req_tx, req_rx) = channel::<ComputeRequest>();
        let (res_tx, res_rx) = channel::<ComputeResult>();

        thread::spawn(move || {
            Self::worker_loop(req_rx, res_tx);
        });

        Self {
            tx: req_tx,
            rx: res_rx,
        }
    }

    fn worker_loop(req_rx: Receiver<ComputeRequest>, res_tx: Sender<ComputeResult>) {
        loop {
            // wait for at least one request
            let mut req = match req_rx.recv() {
                Ok(r) => r,
                Err(_) => return,
            };

            // drain queue
            while let Ok(newer) = req_rx.try_recv() {
                req = newer;
            }

            let mut uncert = None;
            if let Some(red) = req.uncert_reducer {
                uncert = Some(red.reduce(&req.img, req.width, req.height, req.n_colors));
            }
            let mut popula = None;
            if let Some(red) = req.popula_reducer {
                popula = Some(red.reduce(&req.img, req.width, req.height, req.n_colors));
            }
            let mut kmeans = None;
            if let Some(red) = req.kmeans_reducer {
                kmeans = Some(red.reduce(&req.img, req.width, req.height, req.n_colors));
            }

            let _ = res_tx.send(ComputeResult {
                uncert,
                popula,
                kmeans,
                width: req.width,
                height: req.height,
            });
        }
    }
}
