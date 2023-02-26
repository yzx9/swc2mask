use std::{self, fs::File, io::Read};

pub struct V3DPBD {
    mysz: [u32; 4],
}

impl V3DPBD {
    pub fn read(fname: &str) -> Result<V3DPBD, &str> {
        // See also: github.com/Vaa3D/v3d_external/blob/e229df51efafaf8960e0af45c2a5cdd30f18af23/v3d_main/neuron_annotator/utility/ImageLoader.cpp#L940-L1097
        const FORMAT_KEY: &str = "v3d_volume_pkbitdf_encod";
        const HEAD_LEN: usize = FORMAT_KEY.len() + 1 + 2 + 4 * 4;

        let mut f = File::open(fname).or(Err("invalid file"))?;
        let mut buf: [u8; HEAD_LEN] = [0; HEAD_LEN];
        let len = f.read(&mut buf).unwrap_or(0);
        let err = Err("invalid v3dpbd");
        if len != HEAD_LEN || !buf.starts_with(FORMAT_KEY.as_bytes()) {
            return err;
        }

        let endian = buf[FORMAT_KEY.len()];
        let _dcode = &buf[FORMAT_KEY.len() + 1..FORMAT_KEY.len() + 2];

        let i = FORMAT_KEY.len() + 3;
        let mysz = match endian {
            b'L' => [
                u32::from_le_bytes(buf[i + 0 * 4..i + 1 * 4].try_into().unwrap()),
                u32::from_le_bytes(buf[i + 1 * 4..i + 2 * 4].try_into().unwrap()),
                u32::from_le_bytes(buf[i + 2 * 4..i + 3 * 4].try_into().unwrap()),
                u32::from_le_bytes(buf[i + 3 * 4..i + 4 * 4].try_into().unwrap()),
            ],
            b'B' => [
                u32::from_be_bytes(buf[i + 0 * 4..i + 1 * 4].try_into().unwrap()),
                u32::from_be_bytes(buf[i + 1 * 4..i + 2 * 4].try_into().unwrap()),
                u32::from_be_bytes(buf[i + 2 * 4..i + 3 * 4].try_into().unwrap()),
                u32::from_be_bytes(buf[i + 3 * 4..i + 4 * 4].try_into().unwrap()),
            ],
            _ => return err,
        };
        Ok(V3DPBD { mysz })
    }

    pub fn mysz(&self) -> [u32; 4] {
        self.mysz
    }
}
