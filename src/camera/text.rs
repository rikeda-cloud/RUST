use opencv::core::{Mat, Rect, Vector};
use opencv::prelude::BaseOCRTrait;
use opencv::text;

#[allow(dead_code)]
pub fn extract_text(frame: &Mat) -> Result<String, opencv::Error> {
    let mut ocr = text::OCRTesseract::create(
        "/usr/share/tesseract/tessdata/", // システム内のtessdataディレクトリのパス
        "eng",                            // 使用する言語
        "",                               // 文字のホワイトリスト
        3,                                // OCRエンジンモード(OEM)
        10,                               // ページセグメンテーションモード(PSM)
    )?;
    let mut output_text = String::new();
    let mut component_rects = Vector::<Rect>::new();
    let mut component_texts = Vector::<String>::new();
    let mut confidences = Vector::<f32>::new();
    let mut copy_frame = frame.clone();

    // 文字を認識
    ocr.run(
        &mut copy_frame,      // 画像
        &mut output_text,     // 認識結果テキスト
        &mut component_rects, // 認識された文字領域の矩形
        &mut component_texts, // 認識された文字列
        &mut confidences,     // 認識の信頼度
        0,                    // コンポーネントレベル (0 = 文字レベル)
    )?;
    Ok(output_text)
}
