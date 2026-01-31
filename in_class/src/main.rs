fn get_rgb(c:char) -> (u8,u8,u8){

    match c{
        'R' => (255,0,0);
        'G' => (0,255,0);
        'B' => (0,0,255);
    _ => (0,0,0);
    }
    if c == 'R' {
        return (255,0,0);
    }

   (0,0,0)
}

fn main(){

    let letters = ['R', 'G', 'B'];

    for l in letters.iter(){
        let res = get_rgb(*1);
        println!("{:?}", res);
    }


}
