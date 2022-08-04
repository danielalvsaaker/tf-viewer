use async_graphql::{Object, Result};
use tf_database::primitives::ArcBytes;
use tf_models::types::{AngularVelocity, DateTime, Duration, LengthF64, Power, Velocity};

pub struct RecordRoot {
    pub(super) buffer: ArcBytes<'static>,
}

impl RecordRoot {
    async fn inner<T>(&self, field: &'static str) -> Result<T>
    where
        for<'a> T: Send + serde::Deserialize<'a> + 'static,
    {
        let (send, recv) = tokio::sync::oneshot::channel();
        let buffer = self.buffer.clone();

        rayon::spawn(move || {
            let reader = flexbuffers::Reader::get_root(buffer.as_slice())
                .unwrap()
                .as_map()
                .idx(field);
            let res = T::deserialize(reader).unwrap();

            let _ = send.send(res);
        });

        Ok(recv.await?)
    }
}

#[Object(name = "Record")]
impl RecordRoot {
    async fn cadence(&self) -> Result<Vec<Option<AngularVelocity>>> {
        self.inner("cadence").await
    }

    async fn distance(&self) -> Result<Vec<Option<LengthF64>>> {
        self.inner("distance").await
    }

    async fn altitude(&self) -> Result<Vec<Option<LengthF64>>> {
        self.inner("altitude").await
    }

    async fn speed(&self) -> Result<Vec<Option<Velocity>>> {
        self.inner("speed").await
    }

    async fn heartrate(&self) -> Result<Vec<Option<u8>>> {
        self.inner("heartrate").await
    }

    async fn power(&self) -> Result<Vec<Option<Power>>> {
        self.inner("power").await
    }

    async fn lat(&self) -> Result<Vec<Option<f64>>> {
        self.inner("lat").await
    }

    async fn lon(&self) -> Result<Vec<Option<f64>>> {
        self.inner("lon").await
    }

    async fn timestamp(&self) -> Result<Vec<Option<DateTime>>> {
        self.inner("timestamp").await
    }

    async fn duration(&self) -> Result<Vec<Duration>> {
        self.inner("duration").await
    }
}
