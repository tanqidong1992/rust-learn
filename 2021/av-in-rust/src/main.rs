mod test;

use ffmpeg_next as ffmpeg;
use ffmpeg_next::{format, log};
fn main() {

    ffmpeg::init().unwrap();
    log::set_level(log::Level::Warning);
    let inputPath =String::from("test-data/video.mp4");
    let outputPath=String::from("test-output/hls/test.m3u8");
    let mut inputContext =ffmpeg::format::input(&inputPath).unwrap();
    let mut outputContext =ffmpeg::format::output_as(&outputPath, "hls").unwrap();

    let inputVideoStream = find_first_video_stream(&inputContext).unwrap();

    //let encoder=ffmpeg::codec::encoder::find(ffmpeg::codec::Id::None).unwrap();
    let mut outVideoStream=outputContext
        .add_stream(ffmpeg::codec::encoder::find(ffmpeg::codec::Id::None)).unwrap();
    outVideoStream.set_parameters(inputVideoStream.parameters());

    unsafe {
        (*outVideoStream.parameters().as_mut_ptr()).codec_tag=0;
    }
    let outputTimeBase=outVideoStream.time_base();
    let outputStreamIndex =outVideoStream.index();
    let outputTimeBase=outputContext.stream(outputStreamIndex).unwrap().time_base();
    println!("out TimeBase:{}",outputTimeBase);
    outputContext.set_metadata(inputContext.metadata().to_owned());
    outputContext.write_header().unwrap();
    let outputTimeBase=outputContext.stream(outputStreamIndex).unwrap().time_base();
    println!("out TimeBase:{}",outputTimeBase);
    let videoStreamIndex = inputVideoStream.index();
    let mut pts=1;
    for (stream, mut packet) in inputContext.packets(){
        //println!("ist:{} --> ost:{}", stream.index(), videoStreamIndex);
        if stream.index() == videoStreamIndex {
            //let outputTimeBase=outputContext.stream(outputStreamIndex).unwrap().time_base();
           //println!("in timeBase:{},out TimeBase:{}",stream.time_base(),outputTimeBase);
            packet.rescale_ts(stream.time_base(),outputTimeBase);
            packet.set_position(-1);
            packet.set_duration(1);
            packet.set_stream(outputStreamIndex);
            //packet.set_pts(Some(pts));
            //pts+=1;
            //println!("pts {:?}",packet.pts());
            //println!("packet size {:?}",packet.size());
            packet.write_interleaved(& mut outputContext).unwrap();

        }
    }
    outputContext.write_trailer().unwrap();

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
