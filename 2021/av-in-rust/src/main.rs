use ffmpeg_next as ffmpeg;
use ffmpeg_next::format;
fn main() {
    let path=String::from("test-data/video.mp4");
    let mut input =ffmpeg::format::input(&path).expect("Could not read video file");
    let video_stream= find_first_video_stream(&input)
        .expect("");

    let mut outputContext =ffmpeg::format::output_as(&String::from("test-output/hls/test.m3u8"), "hls")
        .expect("Could not allocate hls output context");
    let mut outVideoStream=outputContext.add_stream(video_stream.codec()).expect("");
     outVideoStream.set_parameters(video_stream.parameters());
    unsafe {
        (*outVideoStream.as_mut_ptr()).codec_tag=0;
    }
    outputContext.set_metadata(input.metadata().to_owned());
    outputContext.write_header();

    for (stream, mut packet) in input.packets(){
        if stream.id() == video_stream.id(){
            packet.rescale_ts(stream.time_base(),outVideoStream.time_base());
            packet.set_position(-1);
            packet.set_stream(0);
            packet.write_interleaved(& mut outputContext).unwrap();
        }
    }
    outputContext.write_trailer();
}
fn find_first_video_stream(input:&ffmpeg::format::context::Input) -> Result<ffmpeg::format::stream::Stream,String>{
    let stream_size=input.nb_streams();
    if stream_size <=0 {
        return Result::Err(String::from("Could not found Video Stream"));
    }

    let videoStream=for i in 0..stream_size {
        let stream=input.stream(i as usize).expect("");
        if stream.codec().medium() == ffmpeg::util::media::Type::Video {
            return Result::Ok(stream);
        }
    };
    return Result::Err(String::from("Could not found Video Stream"));
}
