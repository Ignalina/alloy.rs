
trait OneShot {
    fn set_lib(&self) -> f32;
    fn from_chunks(&self) -> f32;


}

struct Bridge {
    ddlname: String,
}

impl OneShot for Bridge {
    fn set_lib(&self) -> f32 {
        1 as f32
//        self.radius.powi(2) * std::f32::consts::PI
    }

    fn from_chunks(&self) -> f32 {

        todo!()
    }
}
