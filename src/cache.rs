use bytes::Bytes;
use moka::future::Cache;
use staticmap::{
    tools::{Color, LineBuilder},
    StaticMapBuilder,
};
use tf_models::activity::Record;

#[derive(Clone)]
pub struct Thumbnail {
    pub data: Bytes,
    pub crc: u32,
}

impl Thumbnail {
    fn new(record: Record) -> Option<Self> {
        let data = Self::generate_thumb(record).map(Bytes::from)?;
        let crc = crc32fast::hash(&data);

        Some(Self { data, crc })
    }

    fn generate_thumb(record: Record) -> Option<Vec<u8>> {
        if record.lon.is_empty() {
            return None;
        }

        let mut map = StaticMapBuilder::default()
            .width(200)
            .height(200)
            .url_template("https://a.tile.openstreetmap.org/{z}/{x}/{y}.png")
            .build()
            .unwrap();

        let line = LineBuilder::default()
            .width(3.)
            .simplify(true)
            .lon_coordinates(record.lon.into_iter().flatten().collect::<Vec<_>>())
            .lat_coordinates(record.lat.into_iter().flatten().collect::<Vec<_>>())
            .color(Color::new(true, 255, 0, 0, 255))
            .tolerance(2.)
            .build()
            .unwrap();

        map.add_tool(line);
        map.encode_png().ok()
    }
}

#[derive(Clone)]
pub struct ThumbnailCache {
    inner: Cache<Vec<u8>, Option<Thumbnail>>,
}

impl ThumbnailCache {
    pub fn new() -> Self {
        Self {
            inner: Cache::new(32),
        }
    }

    pub async fn get(&self, key: Vec<u8>, record: Record) -> Option<Thumbnail> {
        let task = async move {
            let (send, recv) = tokio::sync::oneshot::channel();

            rayon::spawn(move || {
                let _ = send.send(Thumbnail::new(record));
            });

            recv.await.ok().flatten()
        };

        self.inner.get_or_insert_with(key, task).await
    }
}
