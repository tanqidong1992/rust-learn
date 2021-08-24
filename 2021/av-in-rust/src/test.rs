

#[cfg(test)]
mod tests{
    extern crate ffmpeg_next as ffmpeg;

    use std::env;

    use ffmpeg::{codec, encoder, format, log, media, Rational};
    #[test]
    fn main() {
        let input_file = String::from("test-data/video.mp4");
        let output_file = String::from("test-output/hls/test.m3u8");

        ffmpeg::init().unwrap();
        log::set_level(log::Level::Debug);

        let mut ictx = format::input(&input_file).unwrap();
        let mut octx = format::output(&output_file).unwrap();

        let mut stream_mapping = vec![0; ictx.nb_streams() as _];
        let mut ist_time_bases = vec![Rational(0, 1); ictx.nb_streams() as _];
        let mut ost_index = 0;
        for (ist_index, ist) in ictx.streams().enumerate() {
            let ist_medium = ist.codec().medium();
            if ist_medium != media::Type::Audio
                && ist_medium != media::Type::Video
                && ist_medium != media::Type::Subtitle
            {
                stream_mapping[ist_index] = -1;
                continue;
            }
            stream_mapping[ist_index] = ost_index;
            ist_time_bases[ist_index] = ist.time_base();
            ost_index += 1;
            let mut ost = octx.add_stream(encoder::find(codec::Id::None)).unwrap();
            ost.set_parameters(ist.parameters());
            // We need to set codec_tag to 0 lest we run into incompatible codec tag
            // issues when muxing into a different container format. Unfortunately
            // there's no high level API to do this (yet).
            unsafe {
                (*ost.parameters().as_mut_ptr()).codec_tag = 0;
            }
        }

        octx.set_metadata(ictx.metadata().to_owned());
        octx.write_header().unwrap();

        for (stream, mut packet) in ictx.packets() {
            let ist_index = stream.index();
            let ost_index = stream_mapping[ist_index];
            if ost_index < 0 {
                continue;
            }
            let ost = octx.stream(ost_index as _).unwrap();
            packet.rescale_ts(ist_time_bases[ist_index], ost.time_base());
            packet.set_position(-1);
            packet.set_stream(ost_index as _);
            packet.write_interleaved(&mut octx).unwrap();
        }

        octx.write_trailer().unwrap();
    }
}

#[cfg(test)]
mod extract_frame_test{

    use ffmpeg_next::frame::Video;
    use std::io::Write;


    #[test]
    fn transfer_stream(){
        let url=String::from("rtsp://admin:admin12345@192.168.0.199:554/cam/realmonitor?channel=1/subtype=0");
        use ffmpeg_next as ffmpeg;
        let mut input=ffmpeg::format::input(&url).unwrap();
        let stream=input.streams().best(ffmpeg::media::Type::Video)
            .unwrap();
        let video_stream_index=stream.index();
        let mut decoder=stream.codec().decoder().video().unwrap();
        let mut scaler = ffmpeg::software::scaling::Context::get(decoder.format(),
                                                                 decoder.width(),decoder.height(),
                                                                 ffmpeg::format::Pixel::RGB24,decoder.width(),decoder.height(),ffmpeg::software::scaling::Flags::BILINEAR).unwrap();
        let mut frame_index=0;
        let mut receive_and_process_decoded_frames=|decoder:&mut ffmpeg::decoder::Video| -> Result<(),ffmpeg::Error>{
            let mut decoded=ffmpeg::frame::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame=Video::empty();
                scaler.run(&decoded,&mut rgb_frame).unwrap();
                if frame_index>10 && frame_index <20 {
                    save_frame(&rgb_frame, frame_index).unwrap();
                }
                frame_index+=1;
            }
            Ok(())
        };
        for (stream,packet) in input.packets(){
            if stream.index() == video_stream_index{
                decoder.send_packet(&packet).unwrap();
                receive_and_process_decoded_frames(&mut decoder);

            }
        }
    }

    #[test]
    fn test1(){
        use ffmpeg_next as ffmpeg;
        let mut input=ffmpeg::format::input(&"test-data/video.mp4").unwrap();
        let stream=input.streams().best(ffmpeg::media::Type::Video)
            .unwrap();
        let video_stream_index=stream.index();
        let mut decoder=stream.codec().decoder().video().unwrap();
        let mut scaler = ffmpeg::software::scaling::Context::get(decoder.format(),
        decoder.width(),decoder.height(),
        ffmpeg::format::Pixel::RGB24,decoder.width(),decoder.height(),ffmpeg::software::scaling::Flags::BILINEAR).unwrap();
        let mut frame_index=0;
        let mut receive_and_process_decoded_frames=|decoder:&mut ffmpeg::decoder::Video| -> Result<(),ffmpeg::Error>{
            let mut decoded=ffmpeg::frame::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame=Video::empty();
                scaler.run(&decoded,&mut rgb_frame).unwrap();
                if frame_index>10 && frame_index <20 {
                    save_frame(&rgb_frame, frame_index).unwrap();
                }
                frame_index+=1;
            }
            Ok(())
        };
        for (stream,packet) in input.packets(){
            if stream.index() == video_stream_index{
                decoder.send_packet(&packet).unwrap();
                receive_and_process_decoded_frames(&mut decoder);

            }
        }
    }
    fn save_frame(frame:& ffmpeg_next::frame::Video,frame_index:i32)-> Result<(),std::io::Error>{
        let mut file = std::fs::File::create(format!("frame{}.ppm",frame_index)).unwrap();
        file.write_all(format!("P6\n{} {}\n255\n",frame.width(),frame.height()).as_bytes());
        file.write_all(frame.data(0))?;
        Ok(())
    }


}
