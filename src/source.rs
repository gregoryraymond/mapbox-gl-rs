use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GeoJsonSourceSpec<S: Serialize> {
    pub r#type: String,
    pub data: S,
}

impl<S> GeoJsonSourceSpec<S>
where
    S: Serialize,
{
    pub fn new(data: S) -> GeoJsonSourceSpec<S> {
        GeoJsonSourceSpec {
            r#type: "geojson".into(),
            data,
        }
    }
}

pub struct GeoJsonSource {
    pub inner: crate::js::GeoJSONSource,
}

impl GeoJsonSource {
    pub fn set_data(&mut self, data: geojson::GeoJson) -> anyhow::Result<()> {
        let ser = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        //let data = (&source::GeoJsonSourceSpec::new(data))
        //    .serialize(&ser)
        //    .map_err(|_| anyhow::anyhow!("Failed to convert GeoJson"))?;

        self.inner.GeoJSONSource_setData(
            &data
                .serialize(&ser)
                .map_err(|_| anyhow::anyhow!("Failed to convert GeoJson"))?,
        );
        Ok(())
    }
}
