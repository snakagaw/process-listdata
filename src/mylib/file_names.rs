use std::path::PathBuf;

// 
// MT2020_1
//  ├─ raw                                      // 実験で得られた生データ
//  ⎪   ├─ 20220108_se000_add333_0.txt
//  ⎪   ├─ 20220108_se000_add333_1.txt
//  ⎪   ├─ 20220108_se000_add333_2.txt
//  ⎪   ├─ 20220108_se000_add333.lst
//  ⎪   ├─ 20220108_se000_add333.mpa
//  ⎪   ├─ .....
//  ⎪   └─ 20220108_se500_add244_0.txt
//  ├─ 1_COP                                    // 画像を人間が読める形にしたデータ
//  ⎪   ├─ 20220108_se000_add333.txt
//  ⎪   ├─ 20220108_se001_add334.txt
//  ⎪   ├─ .....
//  ⎪   └─ 20220108_se500_add244_0.txt
//  ├─ 1_decoded                                // MCS(二次粒子の測定)を人間が読める形にしたデータ
//  ⎪   ├─ 20220108_se000_add333.lstdecoded
//  ⎪   ├─ 20220108_se001_add334.lstdecoded    TODO: 拡張子をこちらで策定する。atom 使えるなら拡張子は定義しておきたい。
//  ⎪   ├─ .....
//  ⎪   └─ 20220108_se500_add244.lstdecoded
//  ├─ 2_combined                                // imaging, MCSデータを結合したデータ
//  ⎪   ├─ 20220108_se000_add333.txt
//  ⎪   ├─ 20220108_se001_add334.txt
//  ⎪   ├─ .....
//  ⎪   └─ 20220108_se500_add244.txt
//  └─ 3_analyzed                                // 結合データから、スペクトル等を作成したデータ
//      ├─ perpendicular
//      ├─ parallel
//      └─  .....
// TODO: 各ファイルについて、行の仕様を策定する
// TODO: トップディレクトリを指定すれば良いようにする

#[allow(dead_code)]
pub fn lst2lstdecoded(path: &String) -> PathBuf {
    let mut write_path = PathBuf::from(&path);

    let filename = String::from(write_path.file_name().unwrap().to_str().unwrap());
    write_path.pop();
    write_path.push("1_decoded");
    write_path.push(filename);
    write_path.set_extension("lstdecoded");
    write_path
}

#[allow(dead_code)]
pub fn cop2molaxis(path: &String) -> PathBuf {
    let mut write_path = PathBuf::from(&path);

    let filename = String::from(write_path.file_name().unwrap().to_str().unwrap());
    write_path.pop();
    write_path.pop();
    write_path.push("2_molaxis");
    write_path.push(filename);
    write_path
}

#[allow(dead_code)]
pub fn cop2mcs(path: &String) -> PathBuf {
    let mut write_path = PathBuf::from(&path);

    let filename = String::from(write_path.file_name().unwrap().to_str().unwrap());
    write_path.pop();
    write_path.pop();
    write_path.push("1_decoded");
    write_path.push(filename);
    write_path.set_extension("lstdecoded");
    write_path
}

#[allow(dead_code)]
pub fn analyzed_folder(path: &String) -> PathBuf {
    let mut write_path = PathBuf::from(&path);
    write_path.pop(); write_path.pop();
    write_path.push("3_analyze");
    write_path
}