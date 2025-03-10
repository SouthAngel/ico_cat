use std::error::Error;
use std::io::{BufWriter, Write};
use image::DynamicImage;

fn parse_arg_line(_in:&str)->Result<(String,Vec<u32>), Box<dyn Error>>{
    let cs:Vec<char> = _in.chars().collect();
    let mut rpath = String::new();
    let mut ress:Vec<u32> = vec![];
    let mut pl = 1;
    let mut ses = 0;
    let mut see;
    for i in 0.._in.len() {
        let _c = cs[i];
        match _c {
            '<'=>{
                see = i;
                rpath = _in[ses..see].to_string();
                ses = i + 1;
                pl = 2;
            },
            'x'=>{
                see = i;
                if pl==2&&see>ses {
                    let se = _in[ses..see].to_string();
                    if se.cmp(&".".to_string())==std::cmp::Ordering::Equal {
                        ress.push(0);
                    } else {
                        ress.push(u32::from_str_radix(se.as_str(), 10)?);
                    }
                }
                ses = i + 1;
            },
            '>'=>{
                see = i;
                if pl==2&&see>ses {
                    let se = _in[ses..see].to_string();
                    if se.cmp(&".".to_string())==std::cmp::Ordering::Equal {
                        ress.push(0);
                    } else {
                        ress.push(u32::from_str_radix(se.as_str(), 10)?);
                    }
                }
            },
            _=>(),
        }
    }
    if pl == 1 {
        rpath = _in.to_string();
        ress.push(0);
    }
    Ok((rpath, ress))
}

fn parse_args(args:&[String])->Result<Vec<(String,Vec<u32>)>, Box<dyn Error>>{
    // let args: Vec<String> = "ico_cat input1.png input2.png<x.x16x32>".split(" ").map(|x|{x.to_string()}).collect();
    let mut out = vec![];
    for arg in args{
        println!("{}", arg);
        let pa = parse_arg_line(arg.as_str())?;
        out.push(pa);
    }
    Ok(out)
}

fn _main(args:Vec<String>)->Result<(), Box<dyn Error>>{
    let out_path = args.iter().max().cloned().unwrap_or_default();
    let mut outimgs:Vec<DynamicImage> = vec![];
    let mut outimgbs:Vec<Vec<u8>> = vec![];
    for dline in parse_args(&args[1..(&args.len()-1)])?{
        let oimg = image::open(dline.0)?;
        for isz in dline.1{
            if isz == 0 {
                outimgs.push(oimg.clone());
            } else {
                outimgs.push(oimg.resize(isz,isz,image::imageops::FilterType::Triangle));
            }
        }
    }
    // let mut outbs = imgbs.to_vec();
    let mut wf = std::fs::File::options().write(true).truncate(true).create(true).open(out_path)?;
    wf.write(&[0,0,1,0])?;
    wf.write(&(outimgs.len() as u16).to_le_bytes())?;
    for cgimg in &outimgs {
        let mut nv:Vec<u8> = vec![];
        {
            let mut wt = BufWriter::new(&mut nv);
            let pnge = image::codecs::png::PngEncoder::new(&mut wt);
            _ = cgimg.write_with_encoder(pnge);
        }
        outimgbs.push(nv);
    }
    let mut ct = 0;
    let mut ofs:u32 = 6+16*outimgbs.len() as u32;
    for cgimg in &outimgs {
        let _w = cgimg.width();
        let _h = cgimg.height();
        wf.write(&[if _w>256 {0}else{_w as u8},if _h>256 {0}else{_h as u8}])?;
        wf.write(&[0,0])?; // 颜色计数
        wf.write(&(0 as u16).to_le_bytes())?;
        wf.write(&(0 as u16).to_le_bytes())?;
        let blen = outimgbs[ct].len() as u32;
        wf.write(&blen.to_le_bytes())?; // 长度
        wf.write(&ofs.to_le_bytes())?; // 图像偏移
        ofs+=blen;
        ct+=1;
    }
    for cgimg in &outimgbs {
        wf.write(&cgimg)?;
    }
    Ok(())
}

fn pusage(){
    println!("usage:ico_cat input<x..> .. output\nExample:\nico_cat input1.png<x16x32x64> output.ico\nico_cat input1.png input2.png input3.png output.ico\nico_cat input1.png input2.png<x32> output.ico");
}

fn main() {
    println!("ico generate!");
    let args:Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        pusage();
        return;
    }
    match _main(args){
        Ok(_)=>println!("generate completed"),
        Err(e)=>{println!("err {}", e);pusage();},
    }
    println!("The end!");
}
