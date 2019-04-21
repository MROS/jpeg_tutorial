# 跟我寫 JPEG 解碼器

## 真‧緣起
下面這份緣起也不是假的，但爲什麼要寫系列文章，有更直接的原因，如果單純想要學習，就請跳過吧！**唯獨臺灣大學的同學**，請看過這篇文章[吳家麟，我教育你](https://city-of-infinity.com/app/b/%E5%85%AB%E5%8D%A6/a/%E5%90%B3%E5%AE%B6%E9%BA%9F%EF%BC%8C%E6%88%91%E6%95%99%E8%82%B2%E4%BD%A0?id=5cbcd478271ae627b77544ff) （[備份](https://hackmd.io/ctyDPW8dTIuHCaCDkgw-SQ?both)）之後，再繼續閱讀，感謝！

## 緣起
幾年前曾用 C++ 寫過一次 [JPEG 解碼器](https://github.com/MROS/jpeg_decoder)，還記得當時網路上對 JPEG 格式的介紹都雜亂無章、少東缺西，而標準書爲求完整嚴謹，寫的是又臭又長。就沒有讓我趕快寫完作業的法子嗎？最後我在 github 上撈到了一份 python 寫的解碼器，透過追蹤這份程式碼，才好不容易地把網路文章寫的不清楚的地方弄懂了。

我在寫作本文時，特地又搜尋了一次網路文章，發現這幾年確實出現了幾篇比較好的文章，但再繼續深挖，就會發現講解 JPEG 的中文文章萬變不離其宗，都是從這篇 [JPEG 圖像解碼方案](http://read.pudn.com/downloads166/ebook/757412/jpeg/JPEG%CD%BC%CF%F1%BD%E2%C2%EB%B7%BD%B0%B8.pdf)修修補補來的，在理論的深度以及論述的清晰度都略有不足，因此嘗試挑戰看看，能否用自己的方式將 JPEG 講解的更清楚。但有些前人的例子非常不錯，會註明出處後繼續沿用。

本文的目的是：**希冀讀者只要跟着本文的腳步，就能夠最快的打造出自己的 JPEG 解碼器**。此外，我也會在邊寫作本文邊再次實作 JPEG 解碼器的過程中，實作一些能協助除錯的小工具，一併開源讓讀者使用，解碼器的源碼也會盡量保持可讀性，可供讀者直接參考。

如果還有時間，我也會嘗試撰寫一份 JPEG 的理論基礎，畢竟，能夠實作算法，並不代表理解了算法的原理，我當年便是如此，寫完了，但卻感覺沒學到什麼。而理論方面的文章還很缺乏，我願做先鋒，雖千萬人吾往矣。

## 章節說明

爲了便於閱讀，爲了讓讀者有種過關斬將，一直有進度的感覺，我把整套解碼過程切割成五個章節，還有附錄講解 JPEG 的理論基礎、優化技巧。

- [（一）概述](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E4%B8%80%EF%BC%89%E6%A6%82%E8%BF%B0.md)：簡介 JPEG 解碼流程
- [（二）讀取區段](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E4%BA%8C%EF%BC%89%E6%AA%94%E6%A1%88%E7%B5%90%E6%A7%8B.md)：簡介 JPEG 檔案結構
- [（三）讀取量化表、霍夫曼表](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E4%B8%89%EF%BC%89%E8%AE%80%E5%8F%96%E9%87%8F%E5%8C%96%E8%A1%A8%E3%80%81%E9%9C%8D%E5%A4%AB%E6%9B%BC%E8%A1%A8.md)
- [（四）讀取壓縮圖像數據](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E5%9B%9B%EF%BC%89%E8%AE%80%E5%8F%96%E5%A3%93%E7%B8%AE%E5%9C%96%E5%83%8F%E6%95%B8%E6%93%9A.md)
- [（五）解碼](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E4%BA%94%EF%BC%89%E8%A7%A3%E7%A2%BC.md)
- （附錄一）理論基礎
- [（附錄二）優化技巧](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E9%99%84%E9%8C%84%E4%BA%8C%EF%BC%89%E5%84%AA%E5%8C%96%E6%8A%80%E5%B7%A7.md)
- [（附錄三）參考資料](https://github.com/MROS/jpeg_tutorial/blob/master/doc/%E8%B7%9F%E6%88%91%E5%AF%ABjpeg%E8%A7%A3%E7%A2%BC%E5%99%A8%EF%BC%88%E9%99%84%E9%8C%84%E4%B8%89%EF%BC%89%E5%8F%83%E8%80%83%E8%B3%87%E6%96%99.md)

## 閱讀數學式
github 並不支援在 markdown 中寫數學式，建議 clone 本專案之後，以 [typora](https://typora.io/) （在 typora 的偏好設定中開啓行內數學式） 或是其他 markdown 閱讀軟體來閱讀，會有更佳的體驗。

## [配套程式碼](https://github.com/MROS/jpeg_tutorial)

### 前置準備

- 本配套程式碼以 rust 撰寫，請先安裝 [rust 工具鏈](https://www.rust-lang.org/tools/install)。

### 下載程式碼
``` sh
git clone https://github.com/MROS/jpeg_tutorial
```

### 安裝

```sh
cd jpeg_tutorial
cargo install --path .
```
`cargo install` 會將編譯出的執行檔 `jpeg_tutorial` 放進 ~/.cargo/bin 之中，請確認 ~/.cargo/bin 已經在 $PATH 裡。

### 執行

#### 轉檔爲 ppm 格式

ppm 檔的預設檔名爲 out.ppm

``` sh
jpeg_tutorial <jpeg_path> ppm
```

不加子命令，預設效果也是轉換爲 ppm

```sh
jpeg_tutorial <jpeg_path>
```

#### 打印各區段數據

```
jpeg_tutorial <jpeg_path> reader
```

#### 僅打印標記碼

```
jpeg_tutorial <jpeg_path> marker
```

#### 打印指定 mcu 在解碼過程的各階段狀態

```
jpeg_tutorial <jpeg_path> mcu <綜座標> <橫座標>
```

假設有一張圖片高度上有 8 個 mcu  、寬度上有 12 個 mcu ，用

```sh
jpeg_tutorial <jpeg_path> mcu 7 11
```

來得到最右下角的 mcu 各階段狀態，注意到索引從 0 開始。
