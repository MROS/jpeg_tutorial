# 跟我寫 JPEG 解碼器

## 緣起
幾年前曾用 C++ 寫過一次 JPEG 解碼器，還記得當時網路上對 JPEG 格式的介紹都雜亂無章、少東缺西，而標準書爲求完整嚴謹，寫的是又臭又長。就沒有讓我趕快寫完作業的法子嗎？最後我在 github 上撈到了一份 python 寫的解碼器，透過追蹤這份程式碼，才好不容易地把網路文章寫的不清楚的地方弄懂了。

我在寫作本文時，特地又搜尋了一次網路文章，發現這幾年確實出現了幾篇比較好的文章，但再繼續深挖，就會發現各個中文文章萬變不離其宗，基本上都是從這篇[JPEG 圖像解碼方案](http://read.pudn.com/downloads166/ebook/757412/jpeg/JPEG%CD%BC%CF%F1%BD%E2%C2%EB%B7%BD%B0%B8.pdf)修修補補來的，在理論的深度以及論述的清晰度都略有不足，因此嘗試挑戰看看，能否用自己的方式將 JPEG 講解的更清楚。但有些前人的例子非常不錯，會註明出處後繼續沿用。

本文的目的是：**希冀讀者只要跟着本文的腳步，就能夠最快的打造出自己的 JPEG 解碼器**。此外，我也會在邊寫作本文邊再次實作 JPEG 解碼器的過程中，實作一些能協助除錯的小工具，一併開源讓讀者使用，解碼器的源碼也會盡量保持可讀性，可供讀者直接參考。

如果還有時間，我也會嘗試撰寫一份 JPEG 的理論基礎，畢竟，能夠實作算法，並不代表理解了算法的原理，我當年便是如此，寫完了，但卻感覺沒學到什麼。而理論方面的文章還很缺乏，我願做先鋒，雖千萬人吾往矣。

## 章節說明

爲了便於閱讀，爲了讓讀者有種過關斬將，一直有進度的感覺，我把整套解碼過程切割成五個章節，還有一章附錄講解 JPEG 的理論基礎。

- [（一）概述](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E4%B8%80%EF%BC%89%E6%A6%82%E8%BF%B0.md)：簡介 JPEG 解碼流程
- [（二）讀取區段](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E4%BA%8C%EF%BC%89%E6%AA%94%E6%A1%88%E7%B5%90%E6%A7%8B.md)：簡介 JPEG 檔案結構]
- [（三）讀取量化表、霍夫曼表](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E4%B8%89%EF%BC%89%E8%AE%80%E5%8F%96%E9%87%8F%E5%8C%96%E8%A1%A8%E3%80%81%E9%9C%8D%E5%A4%AB%E6%9B%BC%E8%A1%A8.md)
- （四）讀取壓縮圖像數據
- （五）解碼
- （附錄一）理論基礎

## 配套程式碼

### 前置準備

- 本配套程式碼以 rust 撰寫，請先安裝 [rust 工具鏈](https://www.rust-lang.org/tools/install)。
- 安裝 libsdl2 ，通常可在發行版的套件管理器找到

### 下載程式碼
``` sh
git clone https://github.com/MROS/jpeg_tutorial
```

### 執行
``` sh
cd jpeg_tutorial
cargo run XXX.jpg           # 解碼並顯示 jpg 檔
cargo run XXX.jpg --marker  # 於標準輸出打印 jpg 檔的標記碼
```

### 幫助

若想看本程式的完整用法，可以
```
cargo run -- --help
```
