use nutype_enum::nutype_enum;

use crate::ffi::*;

nutype_enum! {
    /// Enum representing various FFmpeg codec IDs.
    ///
    /// Each codec corresponds to an FFmpeg-supported format, including video, audio, and subtitle codecs.
    /// The full list of FFmpeg codecs can be found in the official documentation:
    /// - [FFmpeg Doxygen - avcodec.h](https://ffmpeg.org/doxygen/trunk/avcodec_8h_source.html)
    /// - [FFmpeg Codecs List](https://ffmpeg.org/ffmpeg-codecs.html)
    ///
    /// These IDs are directly mapped from `AV_CODEC_ID_*` constants in FFmpeg.
    pub enum AVCodecID(i32) {
        /// No codec specified.
        None = AV_CODEC_ID_NONE as i32,

        /// MPEG-1 Video codec.
        /// Commonly used in Video CDs and early digital broadcasting.
        Mpeg1Video = AV_CODEC_ID_MPEG1VIDEO as i32,

        /// MPEG-2 Video codec.
        /// Used in DVDs, digital TV broadcasting, and early HD video.
        Mpeg2Video = AV_CODEC_ID_MPEG2VIDEO as i32,

        /// H.261 video codec.
        /// An early video compression standard used for video conferencing.
        H261 = AV_CODEC_ID_H261 as i32,

        /// H.263 video codec.
        /// A predecessor to H.264, used in video conferencing and mobile video.
        H263 = AV_CODEC_ID_H263 as i32,

        /// RealVideo 1.0 codec.
        /// An early proprietary video format from RealNetworks.
        Rv10 = AV_CODEC_ID_RV10 as i32,

        /// RealVideo 2.0 codec.
        /// Improved version of RealVideo for streaming applications.
        Rv20 = AV_CODEC_ID_RV20 as i32,

        /// Motion JPEG codec.
        /// Stores video frames as individual JPEG images.
        Mjpeg = AV_CODEC_ID_MJPEG as i32,

        /// Motion JPEG-B codec.
        /// A variant of Motion JPEG with a slightly different encoding method.
        MjpegB = AV_CODEC_ID_MJPEGB as i32,

        /// Lossless JPEG codec.
        /// Used for medical imaging and other applications needing lossless compression.
        Ljpeg = AV_CODEC_ID_LJPEG as i32,

        /// SP5X codec.
        /// Used in certain digital cameras.
        Sp5X = AV_CODEC_ID_SP5X as i32,

        /// JPEG-LS codec.
        /// A lossless JPEG-based compression format.
        JpegLs = AV_CODEC_ID_JPEGLS as i32,

        /// MPEG-4 Part 2 video codec.
        /// Used in DivX, Xvid, and some early video formats before H.264.
        Mpeg4 = AV_CODEC_ID_MPEG4 as i32,

        /// Raw video codec.
        /// Uncompressed video frames.
        RawVideo = AV_CODEC_ID_RAWVIDEO as i32,

        /// Microsoft MPEG-4 Version 1 codec.
        /// An early proprietary MPEG-4-based codec.
        MsMpeg4V1 = AV_CODEC_ID_MSMPEG4V1 as i32,

        /// Microsoft MPEG-4 Version 2 codec.
        /// Improved version of the earlier Microsoft MPEG-4 codec.
        MsMpeg4V2 = AV_CODEC_ID_MSMPEG4V2 as i32,

        /// Microsoft MPEG-4 Version 3 codec.
        /// Used in older Windows Media Video (WMV) files.
        MsMpeg4V3 = AV_CODEC_ID_MSMPEG4V3 as i32,

        /// Windows Media Video 7 codec.
        /// Early WMV format used for streaming.
        Wmv1 = AV_CODEC_ID_WMV1 as i32,

        /// Windows Media Video 8 codec.
        /// Improved version of WMV1.
        Wmv2 = AV_CODEC_ID_WMV2 as i32,

        /// H.263+ video codec.
        /// An improved version of H.263 with better compression efficiency.
        H263P = AV_CODEC_ID_H263P as i32,

        /// H.263i video codec.
        /// An interlaced variant of H.263.
        H263I = AV_CODEC_ID_H263I as i32,

        /// FLV1 codec.
        /// Used in Adobe Flash Video (.flv) files.
        Flv1 = AV_CODEC_ID_FLV1 as i32,

        /// Sorenson Video 1 codec.
        /// Used in early QuickTime videos.
        Svq1 = AV_CODEC_ID_SVQ1 as i32,

        /// Sorenson Video 3 codec.
        /// A more advanced version used in some QuickTime movies.
        Svq3 = AV_CODEC_ID_SVQ3 as i32,

        /// DV Video codec.
        /// Used in Digital Video (DV) camcorders and professional video production.
        DvVideo = AV_CODEC_ID_DVVIDEO as i32,

        /// HuffYUV codec.
        /// A lossless video compression codec commonly used for archiving.
        Huffyuv = AV_CODEC_ID_HUFFYUV as i32,

        /// Creative Labs YUV codec.
        /// Used in some old hardware-accelerated video capture cards.
        Cyuv = AV_CODEC_ID_CYUV as i32,

        /// H.264 / AVC codec.
        /// One of the most widely used video codecs, offering efficient compression.
        H264 = AV_CODEC_ID_H264 as i32,

        /// Indeo Video 3 codec.
        /// A proprietary video format developed by Intel.
        Indeo3 = AV_CODEC_ID_INDEO3 as i32,

        /// VP3 codec.
        /// A predecessor to Theora, developed by On2 Technologies.
        Vp3 = AV_CODEC_ID_VP3 as i32,

        /// Theora codec.
        /// An open-source video codec based on VP3.
        Theora = AV_CODEC_ID_THEORA as i32,

        /// ASUS Video 1 codec.
        /// Used in ASUS hardware-based video capture solutions.
        Asv1 = AV_CODEC_ID_ASV1 as i32,

        /// ASUS Video 2 codec.
        /// An improved version of ASUS Video 1.
        Asv2 = AV_CODEC_ID_ASV2 as i32,

        /// FFV1 codec.
        /// A lossless video codec developed for archival purposes.
        Ffv1 = AV_CODEC_ID_FFV1 as i32,

        /// 4X Movie codec.
        /// Used in some old video games.
        FourXm = AV_CODEC_ID_4XM as i32,

        /// VCR1 codec.
        /// An early proprietary format for video recording.
        Vcr1 = AV_CODEC_ID_VCR1 as i32,

        /// Cirrus Logic JPEG codec.
        /// Used in certain video capture hardware.
        Cljr = AV_CODEC_ID_CLJR as i32,

        /// MDEC codec.
        /// Used in PlayStation video files.
        Mdec = AV_CODEC_ID_MDEC as i32,

        /// RoQ codec.
        /// Used in some video game cutscenes, notably Quake III.
        Roq = AV_CODEC_ID_ROQ as i32,

        /// Interplay Video codec.
        /// Used in some video game cutscenes from Interplay.
        InterplayVideo = AV_CODEC_ID_INTERPLAY_VIDEO as i32,

        /// Xan WC3 codec.
        /// Used in certain games developed by Westwood Studios.
        XanWc3 = AV_CODEC_ID_XAN_WC3 as i32,

        /// Xan WC4 codec.
        /// An improved version of Xan WC3.
        XanWc4 = AV_CODEC_ID_XAN_WC4 as i32,

        /// RPZA codec.
        /// Used in early Apple QuickTime videos.
        Rpza = AV_CODEC_ID_RPZA as i32,

        /// Cinepak codec.
        /// A widely used video codec in the 1990s for CD-ROM games and early digital videos.
        Cinepak = AV_CODEC_ID_CINEPAK as i32,

        /// Westwood Studios VQA codec.
        /// Used in games developed by Westwood Studios.
        WsVqa = AV_CODEC_ID_WS_VQA as i32,

        /// Microsoft RLE codec.
        /// Used for simple Run-Length Encoding (RLE) video compression.
        MsRle = AV_CODEC_ID_MSRLE as i32,

        /// Microsoft Video 1 codec.
        /// A basic, low-quality video codec used in early Windows applications.
        MsVideo1 = AV_CODEC_ID_MSVIDEO1 as i32,

        /// id CIN codec.
        /// Used in some id Software game cutscenes.
        Idcin = AV_CODEC_ID_IDCIN as i32,

        /// QuickTime 8BPS codec.
        /// A simple video compression format used in QuickTime.
        EightBps = AV_CODEC_ID_8BPS as i32,

        /// Apple Graphics SMC codec.
        /// A very simple codec used in QuickTime.
        Smc = AV_CODEC_ID_SMC as i32,

        /// Autodesk FLIC codec.
        /// Used in animations from Autodesk software.
        Flic = AV_CODEC_ID_FLIC as i32,

        /// TrueMotion 1 codec.
        /// A codec developed by Duck Corporation for video compression.
        Truemotion1 = AV_CODEC_ID_TRUEMOTION1 as i32,

        /// VMD Video codec.
        /// Used in Sierra game cutscenes.
        VmdVideo = AV_CODEC_ID_VMDVIDEO as i32,

        /// Microsoft MSZH codec.
        /// A simple lossless video codec.
        Mszh = AV_CODEC_ID_MSZH as i32,

        /// Zlib codec.
        /// Uses zlib compression for simple lossless video encoding.
        Zlib = AV_CODEC_ID_ZLIB as i32,

        /// QuickTime RLE codec.
        /// A run-length encoding format used in QuickTime movies.
        Qtrle = AV_CODEC_ID_QTRLE as i32,

        /// TechSmith Screen Capture Codec.
        /// Used in Camtasia screen recordings.
        Tscc = AV_CODEC_ID_TSCC as i32,

        /// Ultimotion codec.
        /// Developed by IBM for early digital video.
        Ulti = AV_CODEC_ID_ULTI as i32,

        /// QuickDraw codec.
        /// A legacy codec used in Apple QuickTime.
        Qdraw = AV_CODEC_ID_QDRAW as i32,

        /// VIXL codec.
        /// A lesser-known video codec.
        Vixl = AV_CODEC_ID_VIXL as i32,

        /// QPEG codec.
        /// Used in old video playback software.
        Qpeg = AV_CODEC_ID_QPEG as i32,

        /// PNG codec.
        /// A lossless image format that can also store video sequences.
        Png = AV_CODEC_ID_PNG as i32,

        /// Portable Pixmap (PPM) codec.
        /// A simple, uncompressed image format.
        Ppm = AV_CODEC_ID_PPM as i32,

        /// Portable Bitmap (PBM) codec.
        /// A monochrome image format.
        Pbm = AV_CODEC_ID_PBM as i32,

        /// Portable Graymap (PGM) codec.
        /// A grayscale image format.
        Pgm = AV_CODEC_ID_PGM as i32,

        /// Portable Graymap with YUV format (PGMYUV).
        /// A grayscale format with additional chroma information.
        PgmYuv = AV_CODEC_ID_PGMYUV as i32,

        /// Portable Arbitrary Map (PAM) codec.
        /// A more flexible version of PNM image formats.
        Pam = AV_CODEC_ID_PAM as i32,

        /// FFmpeg Huffman codec.
        /// A lossless video compression format.
        FfvHuff = AV_CODEC_ID_FFVHUFF as i32,

        /// RealVideo 3.0 codec.
        /// Used in RealMedia streaming.
        Rv30 = AV_CODEC_ID_RV30 as i32,

        /// RealVideo 4.0 codec.
        /// An improved version of RealVideo 3.0.
        Rv40 = AV_CODEC_ID_RV40 as i32,

        /// VC-1 codec.
        /// A video codec developed by Microsoft, used in Blu-ray and streaming.
        Vc1 = AV_CODEC_ID_VC1 as i32,

        /// Windows Media Video 9 codec.
        /// Also known as VC-1 Simple/Main profile.
        Wmv3 = AV_CODEC_ID_WMV3 as i32,

        /// LOCO codec.
        /// A low-complexity lossless video codec.
        Loco = AV_CODEC_ID_LOCO as i32,

        /// Winnov WNV1 codec.
        /// Used in some early video capture cards.
        Wnv1 = AV_CODEC_ID_WNV1 as i32,

        /// Autodesk AASC codec.
        /// Used for animation compression in early Autodesk software.
        Aasc = AV_CODEC_ID_AASC as i32,

        /// Indeo Video 2 codec.
        /// A proprietary format from Intel, predating Indeo 3.
        Indeo2 = AV_CODEC_ID_INDEO2 as i32,

        /// Fraps codec.
        /// A lossless codec used in game recording software.
        Fraps = AV_CODEC_ID_FRAPS as i32,

        /// TrueMotion 2 codec.
        /// An improved version of TrueMotion 1, used in older games.
        Truemotion2 = AV_CODEC_ID_TRUEMOTION2 as i32,

        /// BMP codec.
        /// A lossless image format commonly used for raw bitmaps.
        Bmp = AV_CODEC_ID_BMP as i32,

        /// CamStudio codec.
        /// Used in screen recording software.
        Cscd = AV_CODEC_ID_CSCD as i32,

        /// American Laser Games codec.
        /// Used in arcade laserdisc-based games.
        MmVideo = AV_CODEC_ID_MMVIDEO as i32,

        /// DosBox ZMBV codec.
        /// A lossless video codec optimized for DOSBox.
        Zmbv = AV_CODEC_ID_ZMBV as i32,

        /// AVS Video codec.
        /// Used in Chinese digital television broadcasting.
        Avs = AV_CODEC_ID_AVS as i32,

        /// Smacker Video codec.
        /// Used in video game cutscenes.
        SmackVideo = AV_CODEC_ID_SMACKVIDEO as i32,

        /// NuppelVideo codec.
        /// Used in MythTV for recording TV broadcasts.
        Nuv = AV_CODEC_ID_NUV as i32,

        /// Karl Morton's Video Codec.
        /// Used in certain retro multimedia applications.
        Kmvc = AV_CODEC_ID_KMVC as i32,

        /// Flash Screen Video codec.
        /// Used in early versions of Adobe Flash video.
        FlashSv = AV_CODEC_ID_FLASHSV as i32,

        /// Chinese AVS video codec.
        /// Similar to H.264, used in Chinese video applications.
        Cavs = AV_CODEC_ID_CAVS as i32,

        /// JPEG 2000 codec.
        /// A successor to JPEG, offering better compression and quality.
        Jpeg2000 = AV_CODEC_ID_JPEG2000 as i32,

        /// VMware Video codec.
        /// Used in VMware Workstation recordings.
        Vmnc = AV_CODEC_ID_VMNC as i32,

        /// VP5 codec.
        /// A proprietary On2 video codec, predecessor to VP6.
        Vp5 = AV_CODEC_ID_VP5 as i32,

        /// VP6 codec.
        /// A widely used On2 video codec, often found in Flash video.
        Vp6 = AV_CODEC_ID_VP6 as i32,

        /// VP6 Flash codec.
        /// A variant of VP6 optimized for Adobe Flash.
        Vp6F = AV_CODEC_ID_VP6F as i32,

        /// Targa video codec.
        /// Used for storing uncompressed TGA images in video sequences.
        Targa = AV_CODEC_ID_TARGA as i32,

        /// DSICIN Video codec.
        /// Used in games by Westwood Studios.
        DsicinVideo = AV_CODEC_ID_DSICINVIDEO as i32,

        /// Tiertex SEQ Video codec.
        /// Used in old DOS and Amiga video games.
        TiertexSeqVideo = AV_CODEC_ID_TIERTEXSEQVIDEO as i32,

        /// TIFF codec.
        /// A flexible image format supporting both lossless and compressed storage.
        Tiff = AV_CODEC_ID_TIFF as i32,

        /// GIF codec.
        /// Used for simple animations and images with transparency.
        Gif = AV_CODEC_ID_GIF as i32,

        /// DXA codec.
        /// Used in Feeble Files and Broken Sword game cutscenes.
        Dxa = AV_CODEC_ID_DXA as i32,

        /// DNxHD codec.
        /// A professional intermediate codec developed by Avid.
        DnxHd = AV_CODEC_ID_DNXHD as i32,

        /// THP Video codec.
        /// Used in cutscenes on the Nintendo GameCube and Wii.
        Thp = AV_CODEC_ID_THP as i32,

        /// SGI Video codec.
        /// A legacy format used on SGI workstations.
        Sgi = AV_CODEC_ID_SGI as i32,

        /// C93 Video codec.
        /// Used in some Sierra game cutscenes.
        C93 = AV_CODEC_ID_C93 as i32,

        /// Bethesda Softworks Video codec.
        /// Used in older Bethesda games.
        BethSoftVid = AV_CODEC_ID_BETHSOFTVID as i32,

        /// PowerTV PTX codec.
        /// A proprietary video format.
        Ptx = AV_CODEC_ID_PTX as i32,

        /// RenderWare TXD codec.
        /// Used in Grand Theft Auto III and other RenderWare-based games.
        Txd = AV_CODEC_ID_TXD as i32,

        /// VP6A codec.
        /// A variant of VP6 with alpha channel support.
        Vp6A = AV_CODEC_ID_VP6A as i32,

        /// Anime Music Video codec.
        /// A simple codec used for encoding anime clips.
        Amv = AV_CODEC_ID_AMV as i32,

        /// Beam Software VB codec.
        /// Used in older game cutscenes.
        Vb = AV_CODEC_ID_VB as i32,

        /// PCX codec.
        /// A legacy image format from the DOS era.
        Pcx = AV_CODEC_ID_PCX as i32,

        /// Sun Raster Image codec.
        /// A legacy image format from Sun Microsystems.
        Sunrast = AV_CODEC_ID_SUNRAST as i32,

        /// Indeo Video 4 codec.
        /// An improved version of Indeo 3 with better compression.
        Indeo4 = AV_CODEC_ID_INDEO4 as i32,

        /// Indeo Video 5 codec.
        /// A later version of Indeo with better efficiency.
        Indeo5 = AV_CODEC_ID_INDEO5 as i32,

        /// Mimic codec.
        /// Used in certain screen recording applications.
        Mimic = AV_CODEC_ID_MIMIC as i32,

        /// Escape 124 codec.
        /// A proprietary video compression format.
        Escape124 = AV_CODEC_ID_ESCAPE124 as i32,

        /// Dirac codec.
        /// An open-source video codec developed by the BBC.
        Dirac = AV_CODEC_ID_DIRAC as i32,

        /// Bink Video codec.
        /// Used in many game cutscenes.
        BinkVideo = AV_CODEC_ID_BINKVIDEO as i32,

        /// IFF Interleaved Bitmap codec.
        /// Used in Amiga image files.
        IffIlbm = AV_CODEC_ID_IFF_ILBM as i32,

        /// KGV1 codec.
        /// A proprietary video format.
        Kgv1 = AV_CODEC_ID_KGV1 as i32,

        /// YOP Video codec.
        /// Used in some video game cutscenes.
        Yop = AV_CODEC_ID_YOP as i32,

        /// VP8 codec.
        /// A widely used open-source video codec, a predecessor to VP9.
        Vp8 = AV_CODEC_ID_VP8 as i32,

        /// Pictor codec.
        /// Used in early graphic applications.
        Pictor = AV_CODEC_ID_PICTOR as i32,

        /// ANSI Art codec.
        /// Used for text-based animations.
        Ansi = AV_CODEC_ID_ANSI as i32,

        /// A64 Multi codec.
        /// Used for encoding video in the Commodore 64 format.
        A64Multi = AV_CODEC_ID_A64_MULTI as i32,

        /// A64 Multi5 codec.
        /// A variant of A64 Multi with additional encoding options.
        A64Multi5 = AV_CODEC_ID_A64_MULTI5 as i32,

        /// R10K codec.
        /// A high-bit-depth raw video format.
        R10K = AV_CODEC_ID_R10K as i32,

        /// MXPEG codec.
        /// A proprietary codec used in security cameras.
        MxPeg = AV_CODEC_ID_MXPEG as i32,

        /// Lagarith codec.
        /// A lossless video codec used for archival purposes.
        Lagarith = AV_CODEC_ID_LAGARITH as i32,

        /// Apple ProRes codec.
        /// A professional intermediate codec commonly used in video editing.
        ProRes = AV_CODEC_ID_PRORES as i32,

        /// Bitmap Brothers JV codec.
        /// Used in old games for video sequences.
        Jv = AV_CODEC_ID_JV as i32,

        /// DFA codec.
        /// A proprietary format used in some multimedia applications.
        Dfa = AV_CODEC_ID_DFA as i32,

        /// WMV3 Image codec.
        /// A still image format based on Windows Media Video 9.
        Wmv3Image = AV_CODEC_ID_WMV3IMAGE as i32,

        /// VC-1 Image codec.
        /// A still image format based on the VC-1 video codec.
        Vc1Image = AV_CODEC_ID_VC1IMAGE as i32,

        /// Ut Video codec.
        /// A lossless video codec optimized for fast encoding and decoding.
        UtVideo = AV_CODEC_ID_UTVIDEO as i32,

        /// BMV Video codec.
        /// Used in some old video games.
        BmvVideo = AV_CODEC_ID_BMV_VIDEO as i32,

        /// VBLE codec.
        /// A proprietary video compression format.
        Vble = AV_CODEC_ID_VBLE as i32,

        /// Dxtory codec.
        /// Used in game recording software for high-performance capture.
        Dxtory = AV_CODEC_ID_DXTORY as i32,

        /// V410 codec.
        /// A 10-bit YUV 4:4:4 format.
        V410 = AV_CODEC_ID_V410 as i32,

        /// XWD codec.
        /// Used for storing window dumps from the X Window System.
        Xwd = AV_CODEC_ID_XWD as i32,

        /// CDXL codec.
        /// An animation format used on the Commodore Amiga.
        Cdxl = AV_CODEC_ID_CDXL as i32,

        /// XBM codec.
        /// A simple monochrome bitmap format used in X11.
        Xbm = AV_CODEC_ID_XBM as i32,

        /// ZeroCodec.
        /// A lossless video codec used in screen recording.
        ZeroCodec = AV_CODEC_ID_ZEROCODEC as i32,

        /// MSS1 codec.
        /// Microsoft Screen Codec 1, used for remote desktop applications.
        Mss1 = AV_CODEC_ID_MSS1 as i32,

        /// MSA1 codec.
        /// Microsoft Screen Codec 2, an improved version of MSS1.
        Msa1 = AV_CODEC_ID_MSA1 as i32,

        /// TSCC2 codec.
        /// A version of TechSmith Screen Capture Codec.
        Tscc2 = AV_CODEC_ID_TSCC2 as i32,

        /// MTS2 codec.
        /// A proprietary video format.
        Mts2 = AV_CODEC_ID_MTS2 as i32,

        /// CLLC codec.
        /// A proprietary video codec.
        Cllc = AV_CODEC_ID_CLLC as i32,

        /// MSS2 codec.
        /// Microsoft Screen Codec 2, used in Windows Media video recordings.
        Mss2 = AV_CODEC_ID_MSS2 as i32,

        /// VP9 codec.
        /// A popular open-source video codec, successor to VP8.
        Vp9 = AV_CODEC_ID_VP9 as i32,

        /// AIC codec.
        /// Apple Intermediate Codec, used for professional video editing.
        Aic = AV_CODEC_ID_AIC as i32,

        /// Escape 130 codec.
        /// A proprietary video compression format.
        Escape130 = AV_CODEC_ID_ESCAPE130 as i32,

        /// G2M codec.
        /// GoToMeeting screen recording codec.
        G2M = AV_CODEC_ID_G2M as i32,

        /// WebP codec.
        /// A modern image format optimized for the web.
        WebP = AV_CODEC_ID_WEBP as i32,

        /// HNM4 Video codec.
        /// Used in some video game cutscenes.
        Hnm4Video = AV_CODEC_ID_HNM4_VIDEO as i32,

        /// HEVC (H.265) codec.
        /// A high-efficiency video codec, successor to H.264.
        Hevc = AV_CODEC_ID_HEVC as i32,

        /// FIC codec.
        /// A proprietary video compression format.
        Fic = AV_CODEC_ID_FIC as i32,

        /// Alias PIX codec.
        /// Used in old Alias/Wavefront animations.
        AliasPix = AV_CODEC_ID_ALIAS_PIX as i32,

        /// BRender PIX codec.
        /// A proprietary video compression format.
        BRenderPix = AV_CODEC_ID_BRENDER_PIX as i32,

        /// PAF Video codec.
        /// Used in some multimedia applications.
        PafVideo = AV_CODEC_ID_PAF_VIDEO as i32,

        /// OpenEXR codec.
        /// A high-dynamic-range image format used in film production.
        Exr = AV_CODEC_ID_EXR as i32,

        /// VP7 codec.
        /// An older proprietary video codec from On2 Technologies.
        Vp7 = AV_CODEC_ID_VP7 as i32,

        /// SANM codec.
        /// A proprietary video format.
        Sanm = AV_CODEC_ID_SANM as i32,

        /// SGI RLE codec.
        /// A run-length encoding format used on SGI workstations.
        SgiRle = AV_CODEC_ID_SGIRLE as i32,

        /// MVC1 codec.
        /// Multiview Video Coding (MVC) for stereoscopic 3D video.
        Mvc1 = AV_CODEC_ID_MVC1 as i32,

        /// MVC2 codec.
        /// Another variant of Multiview Video Coding.
        Mvc2 = AV_CODEC_ID_MVC2 as i32,

        /// HQX codec.
        /// A high-quality video codec.
        Hqx = AV_CODEC_ID_HQX as i32,

        /// TDSC codec.
        /// A proprietary video compression format.
        Tdsc = AV_CODEC_ID_TDSC as i32,

        /// HQ/HQA codec.
        /// A professional-grade video codec.
        HqHqa = AV_CODEC_ID_HQ_HQA as i32,

        /// HAP codec.
        /// A high-performance video codec for real-time applications.
        Hap = AV_CODEC_ID_HAP as i32,

        /// DDS codec.
        /// A format used for texture compression in graphics applications.
        Dds = AV_CODEC_ID_DDS as i32,

        /// DXV codec.
        /// A proprietary video codec used in Resolume VJ software.
        Dxv = AV_CODEC_ID_DXV as i32,

        /// Screenpresso codec.
        /// A proprietary screen recording codec.
        Screenpresso = AV_CODEC_ID_SCREENPRESSO as i32,

        /// RSCC codec.
        /// A proprietary screen capture codec.
        Rscc = AV_CODEC_ID_RSCC as i32,

        /// AVS2 codec.
        /// A Chinese video codec similar to H.264.
        Avs2 = AV_CODEC_ID_AVS2 as i32,

        /// PGX codec.
        /// A simple image format.
        Pgx = AV_CODEC_ID_PGX as i32,

        /// AVS3 codec.
        /// A next-generation video codec developed in China.
        Avs3 = AV_CODEC_ID_AVS3 as i32,

        /// MSP2 codec.
        /// A proprietary video format.
        Msp2 = AV_CODEC_ID_MSP2 as i32,

        /// VVC codec (H.266).
        /// A next-generation video compression standard.
        Vvc = AV_CODEC_ID_VVC as i32,

        /// Y41P codec.
        /// A planar YUV format.
        Y41P = AV_CODEC_ID_Y41P as i32,

        /// AVRP codec.
        /// A simple video format.
        Avrp = AV_CODEC_ID_AVRP as i32,

        /// 012V codec.
        /// A proprietary video compression format.
        Zero12V = AV_CODEC_ID_012V as i32,

        /// AVUI codec.
        /// A proprietary video format.
        Avui = AV_CODEC_ID_AVUI as i32,

        /// Targa Y216 codec.
        /// A format for storing uncompressed YUV video.
        TargaY216 = AV_CODEC_ID_TARGA_Y216 as i32,

        /// V308 codec.
        /// A planar YUV 4:4:4 format.
        V308 = AV_CODEC_ID_V308 as i32,

        /// V408 codec.
        /// A planar YUV 4:4:4 format with alpha.
        V408 = AV_CODEC_ID_V408 as i32,

        /// YUV4 codec.
        /// A raw YUV video format.
        Yuv4 = AV_CODEC_ID_YUV4 as i32,

        /// AVRN codec.
        /// A proprietary video compression format.
        Avrn = AV_CODEC_ID_AVRN as i32,

        /// CPIA codec.
        /// Used in early webcams.
        Cpia = AV_CODEC_ID_CPIA as i32,

        /// XFace codec.
        /// A low-bandwidth animated face codec.
        XFace = AV_CODEC_ID_XFACE as i32,

        /// Snow codec.
        /// A wavelet-based video codec developed by FFmpeg.
        Snow = AV_CODEC_ID_SNOW as i32,

        /// SMVJPEG codec.
        /// A variant of Motion JPEG.
        SmvJpeg = AV_CODEC_ID_SMVJPEG as i32,

        /// APNG codec.
        /// Animated PNG format.
        Apng = AV_CODEC_ID_APNG as i32,

        /// Daala codec.
        /// An experimental open-source video codec.
        Daala = AV_CODEC_ID_DAALA as i32,

        /// CineForm HD codec.
        /// A professional-grade intermediate codec.
        Cfhd = AV_CODEC_ID_CFHD as i32,

        /// TrueMotion 2RT codec.
        /// A real-time variant of TrueMotion 2.
        Truemotion2Rt = AV_CODEC_ID_TRUEMOTION2RT as i32,

        /// M101 codec.
        /// A proprietary video format.
        M101 = AV_CODEC_ID_M101 as i32,

        /// MagicYUV codec.
        /// A high-performance lossless video codec.
        MagicYuv = AV_CODEC_ID_MAGICYUV as i32,

        /// SheerVideo codec.
        /// A professional-grade lossless video codec.
        SheerVideo = AV_CODEC_ID_SHEERVIDEO as i32,

        /// YLC codec.
        /// A proprietary video compression format.
        Ylc = AV_CODEC_ID_YLC as i32,

        /// PSD codec.
        /// Adobe Photoshop image format.
        Psd = AV_CODEC_ID_PSD as i32,

        /// Pixlet codec.
        /// A video codec developed by Apple for high-performance playback.
        Pixlet = AV_CODEC_ID_PIXLET as i32,

        /// SpeedHQ codec.
        /// A proprietary intermediate codec developed by NewTek.
        SpeedHq = AV_CODEC_ID_SPEEDHQ as i32,

        /// FMVC codec.
        /// A proprietary video format.
        Fmvc = AV_CODEC_ID_FMVC as i32,

        /// SCPR codec.
        /// A screen recording codec.
        Scpr = AV_CODEC_ID_SCPR as i32,

        /// ClearVideo codec.
        /// A wavelet-based video compression format.
        ClearVideo = AV_CODEC_ID_CLEARVIDEO as i32,

        /// XPM codec.
        /// X Pixmap format, used in X Window System.
        Xpm = AV_CODEC_ID_XPM as i32,

        /// AV1 codec.
        /// A modern open-source video codec designed for high compression efficiency.
        Av1 = AV_CODEC_ID_AV1 as i32,

        /// BitPacked codec.
        /// A proprietary bit-packing format.
        BitPacked = AV_CODEC_ID_BITPACKED as i32,

        /// MSCC codec.
        /// A proprietary video format.
        Mscc = AV_CODEC_ID_MSCC as i32,

        /// SRGC codec.
        /// A proprietary video format.
        Srgc = AV_CODEC_ID_SRGC as i32,

        /// SVG codec.
        /// Scalable Vector Graphics format.
        Svg = AV_CODEC_ID_SVG as i32,

        /// GDV codec.
        /// A proprietary video format.
        Gdv = AV_CODEC_ID_GDV as i32,

        /// FITS codec.
        /// Flexible Image Transport System, used in astronomy.
        Fits = AV_CODEC_ID_FITS as i32,

        /// IMM4 codec.
        /// A proprietary video format.
        Imm4 = AV_CODEC_ID_IMM4 as i32,

        /// Prosumer codec.
        /// A proprietary video format.
        Prosumer = AV_CODEC_ID_PROSUMER as i32,

        /// MWSC codec.
        /// A proprietary video format.
        Mwsc = AV_CODEC_ID_MWSC as i32,

        /// WCMV codec.
        /// A proprietary video format.
        Wcmv = AV_CODEC_ID_WCMV as i32,

        /// RASC codec.
        /// A proprietary video format.
        Rasc = AV_CODEC_ID_RASC as i32,

        /// HYMT codec.
        /// A proprietary video compression format.
        Hymt = AV_CODEC_ID_HYMT as i32,

        /// ARBC codec.
        /// A proprietary video format.
        Arbc = AV_CODEC_ID_ARBC as i32,

        /// AGM codec.
        /// A proprietary video format.
        Agm = AV_CODEC_ID_AGM as i32,

        /// LSCR codec.
        /// A proprietary video format.
        Lscr = AV_CODEC_ID_LSCR as i32,

        /// VP4 codec.
        /// An early proprietary video codec from On2 Technologies.
        Vp4 = AV_CODEC_ID_VP4 as i32,

        /// IMM5 codec.
        /// A proprietary video format.
        Imm5 = AV_CODEC_ID_IMM5 as i32,

        /// MVDV codec.
        /// A proprietary video format.
        Mvdv = AV_CODEC_ID_MVDV as i32,

        /// MVHA codec.
        /// A proprietary video format.
        Mvha = AV_CODEC_ID_MVHA as i32,

        /// CDToons codec.
        /// A proprietary video format.
        CdToons = AV_CODEC_ID_CDTOONS as i32,

        /// MV30 codec.
        /// A proprietary video format.
        Mv30 = AV_CODEC_ID_MV30 as i32,

        /// NotchLC codec.
        /// A GPU-accelerated intermediate codec for Notch software.
        NotchLc = AV_CODEC_ID_NOTCHLC as i32,

        /// PFM codec.
        /// Portable FloatMap image format.
        Pfm = AV_CODEC_ID_PFM as i32,

        /// MobiClip codec.
        /// A proprietary video format used in Nintendo DS games.
        MobiClip = AV_CODEC_ID_MOBICLIP as i32,

        /// PhotoCD codec.
        /// A high-quality image format used for storing photographs.
        PhotoCd = AV_CODEC_ID_PHOTOCD as i32,

        /// IPU codec.
        /// Used in PlayStation 2 video playback.
        Ipu = AV_CODEC_ID_IPU as i32,

        /// Argo codec.
        /// A proprietary video format.
        Argo = AV_CODEC_ID_ARGO as i32,

        /// CRI codec.
        /// A proprietary video format used in Japanese games.
        Cri = AV_CODEC_ID_CRI as i32,

        /// Simbiosis IMX codec.
        /// A proprietary video format.
        SimbiosisImx = AV_CODEC_ID_SIMBIOSIS_IMX as i32,

        /// SGA Video codec.
        /// A proprietary video format.
        SgaVideo = AV_CODEC_ID_SGA_VIDEO as i32,

        /// GEM codec.
        /// A proprietary video format.
        Gem = AV_CODEC_ID_GEM as i32,

        /// VBN codec.
        /// A proprietary video format.
        Vbn = AV_CODEC_ID_VBN as i32,

        /// JPEG XL codec.
        /// A modern successor to JPEG with better compression and quality.
        JpegXl = AV_CODEC_ID_JPEGXL as i32,

        /// QOI codec.
        /// Quite OK Image format, a simple lossless image format.
        Qoi = AV_CODEC_ID_QOI as i32,

        /// PHM codec.
        /// A proprietary image format.
        Phm = AV_CODEC_ID_PHM as i32,

        /// Radiance HDR codec.
        /// A high-dynamic-range image format.
        RadianceHdr = AV_CODEC_ID_RADIANCE_HDR as i32,

        /// WBMP codec.
        /// Wireless Bitmap format, used in early mobile applications.
        Wbmp = AV_CODEC_ID_WBMP as i32,

        /// Media100 codec.
        /// A professional video format.
        Media100 = AV_CODEC_ID_MEDIA100 as i32,

        /// VQC codec.
        /// A proprietary video format.
        Vqc = AV_CODEC_ID_VQC as i32,

        /// PDV codec.
        /// A proprietary video format.
        Pdv = AV_CODEC_ID_PDV as i32,

        /// EVC codec.
        /// Essential Video Coding, a next-generation video format.
        Evc = AV_CODEC_ID_EVC as i32,

        /// RTV1 codec.
        /// A proprietary video format.
        Rtv1 = AV_CODEC_ID_RTV1 as i32,

        /// VMIX codec.
        /// A proprietary video format.
        Vmix = AV_CODEC_ID_VMIX as i32,

        /// LEAD codec.
        /// A proprietary video format.
        Lead = AV_CODEC_ID_LEAD as i32,

        /// PCM Signed 16-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmS16Le = AV_CODEC_ID_PCM_S16LE as i32,

        /// PCM Signed 16-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmS16Be = AV_CODEC_ID_PCM_S16BE as i32,

        /// PCM Unsigned 16-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmU16Le = AV_CODEC_ID_PCM_U16LE as i32,

        /// PCM Unsigned 16-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmU16Be = AV_CODEC_ID_PCM_U16BE as i32,

        /// PCM Signed 8-bit codec.
        /// Uncompressed raw audio format.
        PcmS8 = AV_CODEC_ID_PCM_S8 as i32,

        /// PCM Unsigned 8-bit codec.
        /// Uncompressed raw audio format.
        PcmU8 = AV_CODEC_ID_PCM_U8 as i32,

        /// PCM Mu-Law codec.
        /// A logarithmic audio compression format used in telephony.
        PcmMuLaw = AV_CODEC_ID_PCM_MULAW as i32,

        /// PCM A-Law codec.
        /// A logarithmic audio compression format used in telephony.
        PcmALaw = AV_CODEC_ID_PCM_ALAW as i32,

        /// PCM Signed 32-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmS32Le = AV_CODEC_ID_PCM_S32LE as i32,

        /// PCM Signed 32-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmS32Be = AV_CODEC_ID_PCM_S32BE as i32,

        /// PCM Unsigned 32-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmU32Le = AV_CODEC_ID_PCM_U32LE as i32,

        /// PCM Unsigned 32-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmU32Be = AV_CODEC_ID_PCM_U32BE as i32,

        /// PCM Signed 24-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmS24Le = AV_CODEC_ID_PCM_S24LE as i32,

        /// PCM Signed 24-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmS24Be = AV_CODEC_ID_PCM_S24BE as i32,

        /// PCM Unsigned 24-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmU24Le = AV_CODEC_ID_PCM_U24LE as i32,

        /// PCM Unsigned 24-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmU24Be = AV_CODEC_ID_PCM_U24BE as i32,

        /// PCM Signed 24-bit DAUD codec.
        /// Used in digital audio applications.
        PcmS24Daud = AV_CODEC_ID_PCM_S24DAUD as i32,

        /// PCM Zork codec.
        /// A proprietary raw audio format.
        PcmZork = AV_CODEC_ID_PCM_ZORK as i32,

        /// PCM Signed 16-bit Little Endian Planar codec.
        /// Uncompressed raw audio format stored in planar format.
        PcmS16LePlanar = AV_CODEC_ID_PCM_S16LE_PLANAR as i32,

        /// PCM DVD codec.
        /// Used for storing PCM audio in DVD media.
        PcmDvd = AV_CODEC_ID_PCM_DVD as i32,

        /// PCM Floating-Point 32-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmF32Be = AV_CODEC_ID_PCM_F32BE as i32,

        /// PCM Floating-Point 32-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmF32Le = AV_CODEC_ID_PCM_F32LE as i32,

        /// PCM Floating-Point 64-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmF64Be = AV_CODEC_ID_PCM_F64BE as i32,

        /// PCM Floating-Point 64-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmF64Le = AV_CODEC_ID_PCM_F64LE as i32,

        /// PCM Blu-ray codec.
        /// Used in Blu-ray Disc audio.
        PcmBluray = AV_CODEC_ID_PCM_BLURAY as i32,

        /// PCM LXF codec.
        /// Used in Leitch/Harris LXF broadcast video format.
        PcmLxf = AV_CODEC_ID_PCM_LXF as i32,

        /// S302M codec.
        /// Used in professional audio applications.
        S302M = AV_CODEC_ID_S302M as i32,

        /// PCM Signed 8-bit Planar codec.
        /// Uncompressed raw audio stored in planar format.
        PcmS8Planar = AV_CODEC_ID_PCM_S8_PLANAR as i32,

        /// PCM Signed 24-bit Little Endian Planar codec.
        /// Uncompressed raw audio stored in planar format.
        PcmS24LePlanar = AV_CODEC_ID_PCM_S24LE_PLANAR as i32,

        /// PCM Signed 32-bit Little Endian Planar codec.
        /// Uncompressed raw audio stored in planar format.
        PcmS32LePlanar = AV_CODEC_ID_PCM_S32LE_PLANAR as i32,

        /// PCM Signed 16-bit Big Endian Planar codec.
        /// Uncompressed raw audio stored in planar format.
        PcmS16BePlanar = AV_CODEC_ID_PCM_S16BE_PLANAR as i32,

        /// PCM Signed 64-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmS64Le = AV_CODEC_ID_PCM_S64LE as i32,

        /// PCM Signed 64-bit Big Endian codec.
        /// Uncompressed raw audio format.
        PcmS64Be = AV_CODEC_ID_PCM_S64BE as i32,

        /// PCM Floating-Point 16-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmF16Le = AV_CODEC_ID_PCM_F16LE as i32,

        /// PCM Floating-Point 24-bit Little Endian codec.
        /// Uncompressed raw audio format.
        PcmF24Le = AV_CODEC_ID_PCM_F24LE as i32,

        /// PCM VIDC codec.
        /// A proprietary raw audio format.
        PcmVidc = AV_CODEC_ID_PCM_VIDC as i32,

        /// PCM SGA codec.
        /// A proprietary raw audio format.
        PcmSga = AV_CODEC_ID_PCM_SGA as i32,

        /// ADPCM IMA QuickTime codec.
        /// Adaptive Differential Pulse-Code Modulation used in QuickTime.
        AdpcmImaQt = AV_CODEC_ID_ADPCM_IMA_QT as i32,

        /// ADPCM IMA WAV codec.
        /// Adaptive Differential Pulse-Code Modulation used in WAV files.
        AdpcmImaWav = AV_CODEC_ID_ADPCM_IMA_WAV as i32,

        /// ADPCM IMA DK3 codec.
        /// Adaptive Differential Pulse-Code Modulation, variant DK3.
        AdpcmImaDk3 = AV_CODEC_ID_ADPCM_IMA_DK3 as i32,

        /// ADPCM IMA DK4 codec.
        /// Adaptive Differential Pulse-Code Modulation, variant DK4.
        AdpcmImaDk4 = AV_CODEC_ID_ADPCM_IMA_DK4 as i32,

        /// ADPCM IMA Westwood codec.
        /// Used in Westwood Studios video games.
        AdpcmImaWs = AV_CODEC_ID_ADPCM_IMA_WS as i32,

        /// ADPCM IMA Smacker codec.
        /// Used in Smacker video format.
        AdpcmImaSmjpeg = AV_CODEC_ID_ADPCM_IMA_SMJPEG as i32,

        /// ADPCM Microsoft codec.
        /// Microsoft variant of Adaptive Differential Pulse-Code Modulation.
        AdpcmMs = AV_CODEC_ID_ADPCM_MS as i32,

        /// ADPCM 4X Movie codec.
        /// Used in 4X Movie video format.
        Adpcm4Xm = AV_CODEC_ID_ADPCM_4XM as i32,

        /// ADPCM XA codec.
        /// Used in PlayStation XA audio format.
        AdpcmXa = AV_CODEC_ID_ADPCM_XA as i32,

        /// ADPCM ADX codec.
        /// Used in ADX audio format, common in Sega games.
        AdpcmAdx = AV_CODEC_ID_ADPCM_ADX as i32,

        /// ADPCM Electronic Arts codec.
        /// Used in Electronic Arts games.
        AdpcmEa = AV_CODEC_ID_ADPCM_EA as i32,

        /// ADPCM G.726 codec.
        /// ITU-T standard for speech compression.
        AdpcmG726 = AV_CODEC_ID_ADPCM_G726 as i32,

        /// ADPCM Creative codec.
        /// Used in Creative Labs sound hardware.
        AdpcmCt = AV_CODEC_ID_ADPCM_CT as i32,

        /// ADPCM SWF codec.
        /// Used in Adobe Flash audio.
        AdpcmSwf = AV_CODEC_ID_ADPCM_SWF as i32,

        /// ADPCM Yamaha codec.
        /// A variant of ADPCM used in Yamaha audio applications.
        AdpcmYamaha = AV_CODEC_ID_ADPCM_YAMAHA as i32,

        /// ADPCM Sound Blaster Pro 4-bit codec.
        /// Used in Sound Blaster Pro hardware.
        AdpcmSbpro4 = AV_CODEC_ID_ADPCM_SBPRO_4 as i32,

        /// ADPCM Sound Blaster Pro 3-bit codec.
        /// Used in Sound Blaster Pro hardware.
        AdpcmSbpro3 = AV_CODEC_ID_ADPCM_SBPRO_3 as i32,

        /// ADPCM Sound Blaster Pro 2-bit codec.
        /// Used in Sound Blaster Pro hardware.
        AdpcmSbpro2 = AV_CODEC_ID_ADPCM_SBPRO_2 as i32,

        /// ADPCM THP codec.
        /// Used in Nintendo THP video files.
        AdpcmThp = AV_CODEC_ID_ADPCM_THP as i32,

        /// ADPCM IMA AMV codec.
        /// Used in AMV video format.
        AdpcmImaAmv = AV_CODEC_ID_ADPCM_IMA_AMV as i32,

        /// ADPCM Electronic Arts R1 codec.
        /// Used in EA games.
        AdpcmEaR1 = AV_CODEC_ID_ADPCM_EA_R1 as i32,

        /// ADPCM Electronic Arts R3 codec.
        /// Used in EA games.
        AdpcmEaR3 = AV_CODEC_ID_ADPCM_EA_R3 as i32,

        /// ADPCM Electronic Arts R2 codec.
        /// Used in EA games.
        AdpcmEaR2 = AV_CODEC_ID_ADPCM_EA_R2 as i32,

        /// ADPCM IMA Electronic Arts SEAD codec.
        /// Used in Electronic Arts games.
        AdpcmImaEaSead = AV_CODEC_ID_ADPCM_IMA_EA_SEAD as i32,

        /// ADPCM IMA Electronic Arts EACS codec.
        /// Used in Electronic Arts games.
        AdpcmImaEaEacs = AV_CODEC_ID_ADPCM_IMA_EA_EACS as i32,

        /// ADPCM Electronic Arts XAS codec.
        /// Used in Electronic Arts games.
        AdpcmEaXas = AV_CODEC_ID_ADPCM_EA_XAS as i32,

        /// ADPCM Electronic Arts Maxis XA codec.
        /// Used in Maxis-developed games.
        AdpcmEaMaxisXa = AV_CODEC_ID_ADPCM_EA_MAXIS_XA as i32,

        /// ADPCM IMA ISS codec.
        /// Used in ISS audio format.
        AdpcmImaIss = AV_CODEC_ID_ADPCM_IMA_ISS as i32,

        /// ADPCM G.722 codec.
        /// Used in telephony applications.
        AdpcmG722 = AV_CODEC_ID_ADPCM_G722 as i32,

        /// ADPCM IMA APC codec.
        /// A proprietary ADPCM format.
        AdpcmImaApc = AV_CODEC_ID_ADPCM_IMA_APC as i32,

        /// ADPCM VIMA codec.
        /// A proprietary ADPCM format.
        AdpcmVima = AV_CODEC_ID_ADPCM_VIMA as i32,

        /// ADPCM AFC codec.
        /// A proprietary ADPCM format.
        AdpcmAfc = AV_CODEC_ID_ADPCM_AFC as i32,

        /// ADPCM IMA OKI codec.
        /// A proprietary ADPCM format.
        AdpcmImaOki = AV_CODEC_ID_ADPCM_IMA_OKI as i32,

        /// ADPCM DTK codec.
        /// Used in some proprietary applications.
        AdpcmDtk = AV_CODEC_ID_ADPCM_DTK as i32,

        /// ADPCM IMA RAD codec.
        /// A proprietary ADPCM format.
        AdpcmImaRad = AV_CODEC_ID_ADPCM_IMA_RAD as i32,

        /// ADPCM G.726LE codec.
        /// A variant of G.726 with little-endian encoding.
        AdpcmG726Le = AV_CODEC_ID_ADPCM_G726LE as i32,

        /// ADPCM THP LE codec.
        /// Used in Nintendo THP files with little-endian storage.
        AdpcmThpLe = AV_CODEC_ID_ADPCM_THP_LE as i32,

        /// ADPCM PlayStation codec.
        /// Used in PlayStation audio formats.
        AdpcmPsx = AV_CODEC_ID_ADPCM_PSX as i32,

        /// ADPCM AICA codec.
        /// Used in Sega Dreamcast AICA sound chip.
        AdpcmAica = AV_CODEC_ID_ADPCM_AICA as i32,

        /// ADPCM IMA DAT4 codec.
        /// A proprietary ADPCM format.
        AdpcmImaDat4 = AV_CODEC_ID_ADPCM_IMA_DAT4 as i32,

        /// ADPCM MTAF codec.
        /// A proprietary ADPCM format.
        AdpcmMtaf = AV_CODEC_ID_ADPCM_MTAF as i32,

        /// ADPCM AGM codec.
        /// A proprietary ADPCM format.
        AdpcmAgm = AV_CODEC_ID_ADPCM_AGM as i32,

        /// ADPCM Argo codec.
        /// A proprietary ADPCM format.
        AdpcmArgo = AV_CODEC_ID_ADPCM_ARGO as i32,

        /// ADPCM IMA SSI codec.
        /// A proprietary ADPCM format.
        AdpcmImaSsi = AV_CODEC_ID_ADPCM_IMA_SSI as i32,

        /// ADPCM Zork codec.
        /// A proprietary ADPCM format used in Zork games.
        AdpcmZork = AV_CODEC_ID_ADPCM_ZORK as i32,

        /// ADPCM IMA APM codec.
        /// A proprietary ADPCM format.
        AdpcmImaApm = AV_CODEC_ID_ADPCM_IMA_APM as i32,

        /// ADPCM IMA ALP codec.
        /// A proprietary ADPCM format.
        AdpcmImaAlp = AV_CODEC_ID_ADPCM_IMA_ALP as i32,

        /// ADPCM IMA MTF codec.
        /// A proprietary ADPCM format.
        AdpcmImaMtf = AV_CODEC_ID_ADPCM_IMA_MTF as i32,

        /// ADPCM IMA Cunning codec.
        /// A proprietary ADPCM format.
        AdpcmImaCunning = AV_CODEC_ID_ADPCM_IMA_CUNNING as i32,

        /// ADPCM IMA Moflex codec.
        /// Used in Moflex multimedia format.
        AdpcmImaMoflex = AV_CODEC_ID_ADPCM_IMA_MOFLEX as i32,

        /// ADPCM IMA Acorn codec.
        /// A proprietary ADPCM format.
        AdpcmImaAcorn = AV_CODEC_ID_ADPCM_IMA_ACORN as i32,

        /// ADPCM XMD codec.
        /// A proprietary ADPCM format.
        AdpcmXmd = AV_CODEC_ID_ADPCM_XMD as i32,

        /// AMR Narrowband codec.
        /// Adaptive Multi-Rate codec, used in mobile telephony.
        AmrNb = AV_CODEC_ID_AMR_NB as i32,

        /// AMR Wideband codec.
        /// A higher-quality variant of AMR.
        AmrWb = AV_CODEC_ID_AMR_WB as i32,

        /// RealAudio 1.44 kbps codec.
        /// Used in RealMedia audio streams.
        Ra144 = AV_CODEC_ID_RA_144 as i32,

        /// RealAudio 2.88 kbps codec.
        /// Used in RealMedia audio streams.
        Ra288 = AV_CODEC_ID_RA_288 as i32,

        /// RoQ DPCM codec.
        /// Used in video game audio, notably Quake III.
        RoqDpcm = AV_CODEC_ID_ROQ_DPCM as i32,

        /// Interplay DPCM codec.
        /// Used in Interplay Entertainment video game audio.
        InterplayDpcm = AV_CODEC_ID_INTERPLAY_DPCM as i32,

        /// Xan DPCM codec.
        /// Used in certain Xan-based multimedia formats.
        XanDpcm = AV_CODEC_ID_XAN_DPCM as i32,

        /// Sol DPCM codec.
        /// Used in some multimedia applications.
        SolDpcm = AV_CODEC_ID_SOL_DPCM as i32,

        /// SDX2 DPCM codec.
        /// A proprietary DPCM format.
        Sdx2Dpcm = AV_CODEC_ID_SDX2_DPCM as i32,

        /// Gremlin DPCM codec.
        /// Used in Gremlin Interactive games.
        GremlinDpcm = AV_CODEC_ID_GREMLIN_DPCM as i32,

        /// DERF DPCM codec.
        /// A proprietary DPCM format.
        DerfDpcm = AV_CODEC_ID_DERF_DPCM as i32,

        /// WADY DPCM codec.
        /// A proprietary DPCM format.
        WadyDpcm = AV_CODEC_ID_WADY_DPCM as i32,

        /// CBD2 DPCM codec.
        /// A proprietary DPCM format.
        Cbd2Dpcm = AV_CODEC_ID_CBD2_DPCM as i32,

        /// MP2 codec.
        /// MPEG Audio Layer II, commonly used in digital radio and TV.
        Mp2 = AV_CODEC_ID_MP2 as i32,

        /// MP3 codec.
        /// MPEG Audio Layer III, one of the most popular audio formats.
        Mp3 = AV_CODEC_ID_MP3 as i32,

        /// AAC codec.
        /// Advanced Audio Coding, widely used in streaming and mobile applications.
        Aac = AV_CODEC_ID_AAC as i32,

        /// AC3 codec.
        /// Dolby Digital audio codec, used in DVDs and broadcasting.
        Ac3 = AV_CODEC_ID_AC3 as i32,

        /// DTS codec.
        /// Digital Theater Systems audio, commonly used in Blu-ray and cinema.
        Dts = AV_CODEC_ID_DTS as i32,

        /// Vorbis codec.
        /// A free, open-source audio codec.
        Vorbis = AV_CODEC_ID_VORBIS as i32,

        /// DV Audio codec.
        /// Used in Digital Video (DV) camcorders.
        DvAudio = AV_CODEC_ID_DVAUDIO as i32,

        /// Windows Media Audio v1 codec.
        /// Early version of WMA format.
        WmaV1 = AV_CODEC_ID_WMAV1 as i32,

        /// Windows Media Audio v2 codec.
        /// An improved version of WMA.
        WmaV2 = AV_CODEC_ID_WMAV2 as i32,

        /// MACE 3 codec.
        /// Used in old Macintosh applications.
        Mace3 = AV_CODEC_ID_MACE3 as i32,

        /// MACE 6 codec.
        /// A higher compression variant of MACE 3.
        Mace6 = AV_CODEC_ID_MACE6 as i32,

        /// VMD Audio codec.
        /// Used in Sierra VMD multimedia format.
        VmdAudio = AV_CODEC_ID_VMDAUDIO as i32,

        /// FLAC codec.
        /// Free Lossless Audio Codec, widely used for high-quality audio storage.
        Flac = AV_CODEC_ID_FLAC as i32,

        /// MP3 ADU codec.
        /// A variant of MP3 optimized for streaming.
        Mp3Adu = AV_CODEC_ID_MP3ADU as i32,

        /// MP3-on-MP4 codec.
        /// MP3 audio stored in an MP4 container.
        Mp3On4 = AV_CODEC_ID_MP3ON4 as i32,

        /// Shorten codec.
        /// A lossless audio compression format.
        Shorten = AV_CODEC_ID_SHORTEN as i32,

        /// ALAC codec.
        /// Apple Lossless Audio Codec, used in iTunes and Apple devices.
        Alac = AV_CODEC_ID_ALAC as i32,

        /// Westwood SND1 codec.
        /// Used in Westwood Studios games.
        WestwoodSnd1 = AV_CODEC_ID_WESTWOOD_SND1 as i32,

        /// GSM codec.
        /// A low-bitrate speech codec used in mobile networks.
        Gsm = AV_CODEC_ID_GSM as i32,

        /// QDM2 codec.
        /// Used in older QuickTime audio formats.
        Qdm2 = AV_CODEC_ID_QDM2 as i32,

        /// Cook codec.
        /// A proprietary RealAudio format.
        Cook = AV_CODEC_ID_COOK as i32,

        /// TrueSpeech codec.
        /// A low-bitrate speech codec developed by DSP Group.
        TrueSpeech = AV_CODEC_ID_TRUESPEECH as i32,

        /// TTA codec.
        /// The True Audio codec, a lossless compression format.
        Tta = AV_CODEC_ID_TTA as i32,

        /// Smacker Audio codec.
        /// Used in Smacker video files.
        SmackAudio = AV_CODEC_ID_SMACKAUDIO as i32,

        /// QCELP codec.
        /// Qualcomm's PureVoice codec, used in early mobile phones.
        Qcelp = AV_CODEC_ID_QCELP as i32,

        /// WavPack codec.
        /// A lossless and hybrid audio compression format.
        WavPack = AV_CODEC_ID_WAVPACK as i32,

        /// Discworld II Audio codec.
        /// Used in certain FMV-based video games.
        DsicinAudio = AV_CODEC_ID_DSICINAUDIO as i32,

        /// IMC codec.
        /// Intel Music Coder, a proprietary speech codec.
        Imc = AV_CODEC_ID_IMC as i32,

        /// Musepack v7 codec.
        /// A lossy audio format optimized for high-quality compression.
        Musepack7 = AV_CODEC_ID_MUSEPACK7 as i32,

        /// MLP codec.
        /// Meridian Lossless Packing, used in high-definition audio.
        Mlp = AV_CODEC_ID_MLP as i32,

        /// GSM Microsoft codec.
        /// A variant of GSM used in Microsoft applications.
        GsmMs = AV_CODEC_ID_GSM_MS as i32,

        /// ATRAC3 codec.
        /// Sony's Adaptive Transform Acoustic Coding, used in MiniDisc and PSP.
        Atrac3 = AV_CODEC_ID_ATRAC3 as i32,

        /// APE codec.
        /// Monkey's Audio, a lossless audio format.
        Ape = AV_CODEC_ID_APE as i32,

        /// Nellymoser codec.
        /// Used in Flash-based streaming audio.
        Nellymoser = AV_CODEC_ID_NELLYMOSER as i32,

        /// Musepack v8 codec.
        /// A newer version of the Musepack audio format.
        Musepack8 = AV_CODEC_ID_MUSEPACK8 as i32,

        /// Speex codec.
        /// A speech codec optimized for low bitrate applications.
        Speex = AV_CODEC_ID_SPEEX as i32,

        /// Windows Media Audio Voice codec.
        /// Used for low-bitrate speech in Windows Media applications.
        WmaVoice = AV_CODEC_ID_WMAVOICE as i32,

        /// Windows Media Audio Professional codec.
        /// A high-fidelity version of Windows Media Audio.
        WmaPro = AV_CODEC_ID_WMAPRO as i32,

        /// Windows Media Audio Lossless codec.
        /// A lossless compression format from Microsoft.
        WmaLossless = AV_CODEC_ID_WMALOSSLESS as i32,

        /// ATRAC3+ codec.
        /// An improved version of Sony's ATRAC3 format.
        Atrac3P = AV_CODEC_ID_ATRAC3P as i32,

        /// Enhanced AC-3 codec.
        /// Also known as E-AC-3, used in digital broadcasting and Blu-ray.
        Eac3 = AV_CODEC_ID_EAC3 as i32,

        /// SIPR codec.
        /// A proprietary RealAudio codec.
        Sipr = AV_CODEC_ID_SIPR as i32,

        /// MP1 codec.
        /// MPEG Audio Layer I, an early form of MP2/MP3.
        Mp1 = AV_CODEC_ID_MP1 as i32,

        /// TwinVQ codec.
        /// A low-bitrate audio codec developed by NTT.
        TwinVq = AV_CODEC_ID_TWINVQ as i32,

        /// TrueHD codec.
        /// A lossless audio format used in Blu-ray.
        TrueHd = AV_CODEC_ID_TRUEHD as i32,

        /// MPEG-4 ALS codec.
        /// A lossless audio codec in the MPEG-4 standard.
        Mp4Als = AV_CODEC_ID_MP4ALS as i32,

        /// ATRAC1 codec.
        /// The original Adaptive Transform Acoustic Coding format from Sony.
        Atrac1 = AV_CODEC_ID_ATRAC1 as i32,

        /// Bink Audio RDFT codec.
        /// Used in Bink video files.
        BinkAudioRdft = AV_CODEC_ID_BINKAUDIO_RDFT as i32,

        /// Bink Audio DCT codec.
        /// Another audio format used in Bink multimedia.
        BinkAudioDct = AV_CODEC_ID_BINKAUDIO_DCT as i32,

        /// AAC LATM codec.
        /// A variant of AAC used in transport streams.
        AacLatm = AV_CODEC_ID_AAC_LATM as i32,

        /// QDMC codec.
        /// A proprietary QuickTime audio format.
        Qdmc = AV_CODEC_ID_QDMC as i32,

        /// CELT codec.
        /// A low-latency audio codec, later integrated into Opus.
        Celt = AV_CODEC_ID_CELT as i32,

        /// G.723.1 codec.
        /// A speech codec used in VoIP applications.
        G723_1 = AV_CODEC_ID_G723_1 as i32,

        /// G.729 codec.
        /// A low-bitrate speech codec commonly used in telephony.
        G729 = AV_CODEC_ID_G729 as i32,

        /// 8SVX Exponential codec.
        /// An audio format used on Amiga computers.
        EightSvxExp = AV_CODEC_ID_8SVX_EXP as i32,

        /// 8SVX Fibonacci codec.
        /// Another variant of the 8SVX Amiga audio format.
        EightSvxFib = AV_CODEC_ID_8SVX_FIB as i32,

        /// BMV Audio codec.
        /// Used in multimedia applications.
        BmvAudio = AV_CODEC_ID_BMV_AUDIO as i32,

        /// RALF codec.
        /// A proprietary RealAudio format.
        Ralf = AV_CODEC_ID_RALF as i32,

        /// IAC codec.
        /// An obscure proprietary format.
        Iac = AV_CODEC_ID_IAC as i32,

        /// iLBC codec.
        /// Internet Low Bitrate Codec, used in VoIP.
        Ilbc = AV_CODEC_ID_ILBC as i32,

        /// Opus codec.
        /// A highly efficient and low-latency audio codec for streaming and VoIP.
        Opus = AV_CODEC_ID_OPUS as i32,

        /// Comfort Noise codec.
        /// Used in VoIP applications to generate artificial background noise.
        ComfortNoise = AV_CODEC_ID_COMFORT_NOISE as i32,

        /// TAK codec.
        /// A lossless audio compression format.
        Tak = AV_CODEC_ID_TAK as i32,

        /// MetaSound codec.
        /// A proprietary audio format.
        MetaSound = AV_CODEC_ID_METASOUND as i32,

        /// PAF Audio codec.
        /// Used in some multimedia applications.
        PafAudio = AV_CODEC_ID_PAF_AUDIO as i32,

        /// On2 AVC codec.
        /// A proprietary format from On2 Technologies.
        On2Avc = AV_CODEC_ID_ON2AVC as i32,

        /// DSS SP codec.
        /// Used in digital dictation software.
        DssSp = AV_CODEC_ID_DSS_SP as i32,

        /// Codec2 codec.
        /// A very low-bitrate speech codec for radio communications.
        Codec2 = AV_CODEC_ID_CODEC2 as i32,

        /// FFmpeg WaveSynth codec.
        /// A synthetic waveform generator.
        FfwaveSynth = AV_CODEC_ID_FFWAVESYNTH as i32,

        /// Sonic codec.
        /// An experimental lossy audio format.
        Sonic = AV_CODEC_ID_SONIC as i32,

        /// Sonic LS codec.
        /// A lossless version of Sonic.
        SonicLs = AV_CODEC_ID_SONIC_LS as i32,

        /// EVRC codec.
        /// A speech codec used in CDMA networks.
        Evrc = AV_CODEC_ID_EVRC as i32,

        /// SMV codec.
        /// A speech codec for mobile networks.
        Smv = AV_CODEC_ID_SMV as i32,

        /// DSD LSBF codec.
        /// Direct Stream Digital format with least-significant-bit first ordering.
        DsdLsbf = AV_CODEC_ID_DSD_LSBF as i32,

        /// DSD MSBF codec.
        /// Direct Stream Digital format with most-significant-bit first ordering.
        DsdMsbf = AV_CODEC_ID_DSD_MSBF as i32,

        /// DSD LSBF Planar codec.
        /// Planar version of DSD LSBF.
        DsdLsbfPlanar = AV_CODEC_ID_DSD_LSBF_PLANAR as i32,

        /// DSD MSBF Planar codec.
        /// Planar version of DSD MSBF.
        DsdMsbfPlanar = AV_CODEC_ID_DSD_MSBF_PLANAR as i32,

        /// 4GV codec.
        /// A speech codec used in cellular networks.
        FourGv = AV_CODEC_ID_4GV as i32,

        /// Interplay ACM codec.
        /// Used in Interplay Entertainment video games.
        InterplayAcm = AV_CODEC_ID_INTERPLAY_ACM as i32,

        /// XMA1 codec.
        /// Xbox Media Audio version 1.
        Xma1 = AV_CODEC_ID_XMA1 as i32,

        /// XMA2 codec.
        /// Xbox Media Audio version 2.
        Xma2 = AV_CODEC_ID_XMA2 as i32,

        /// DST codec.
        /// Direct Stream Transfer, used in Super Audio CDs.
        Dst = AV_CODEC_ID_DST as i32,

        /// ATRAC3AL codec.
        /// A variant of ATRAC3 used in some Sony devices.
        Atrac3Al = AV_CODEC_ID_ATRAC3AL as i32,

        /// ATRAC3PAL codec.
        /// A variant of ATRAC3 used in some Sony devices.
        Atrac3Pal = AV_CODEC_ID_ATRAC3PAL as i32,

        /// Dolby E codec.
        /// Used in professional broadcast audio.
        DolbyE = AV_CODEC_ID_DOLBY_E as i32,

        /// aptX codec.
        /// A Bluetooth audio codec optimized for high quality.
        Aptx = AV_CODEC_ID_APTX as i32,

        /// aptX HD codec.
        /// A higher-quality version of aptX.
        AptxHd = AV_CODEC_ID_APTX_HD as i32,

        /// SBC codec.
        /// A standard Bluetooth audio codec.
        Sbc = AV_CODEC_ID_SBC as i32,

        /// ATRAC9 codec.
        /// A high-efficiency Sony audio codec used in PlayStation consoles.
        Atrac9 = AV_CODEC_ID_ATRAC9 as i32,

        /// HCOM codec.
        /// A proprietary audio compression format.
        Hcom = AV_CODEC_ID_HCOM as i32,

        /// ACELP Kelvin codec.
        /// A speech codec.
        AcelpKelvin = AV_CODEC_ID_ACELP_KELVIN as i32,

        /// MPEG-H 3D Audio codec.
        /// A next-generation audio standard with 3D sound.
        Mpegh3DAudio = AV_CODEC_ID_MPEGH_3D_AUDIO as i32,

        /// Siren codec.
        /// A speech codec used in VoIP.
        Siren = AV_CODEC_ID_SIREN as i32,

        /// HCA codec.
        /// A proprietary format used in Japanese games.
        Hca = AV_CODEC_ID_HCA as i32,

        /// FastAudio codec.
        /// A proprietary format.
        FastAudio = AV_CODEC_ID_FASTAUDIO as i32,

        /// MSN Siren codec.
        /// Used in older MSN Messenger voice communication.
        MsnSiren = AV_CODEC_ID_MSNSIREN as i32,

        /// DFPWM codec.
        /// A low-bitrate waveform compression format.
        Dfpwm = AV_CODEC_ID_DFPWM as i32,

        /// Bonk codec.
        /// A lossy audio compression format.
        Bonk = AV_CODEC_ID_BONK as i32,

        /// Misc4 codec.
        /// A proprietary audio format.
        Misc4 = AV_CODEC_ID_MISC4 as i32,

        /// APAC codec.
        /// A proprietary audio format.
        Apac = AV_CODEC_ID_APAC as i32,

        /// FTR codec.
        /// A proprietary audio format.
        Ftr = AV_CODEC_ID_FTR as i32,

        /// WAVARC codec.
        /// A proprietary audio format.
        WavArc = AV_CODEC_ID_WAVARC as i32,

        /// RKA codec.
        /// A proprietary audio format.
        Rka = AV_CODEC_ID_RKA as i32,

        /// AC4 codec.
        /// A next-generation Dolby audio codec for broadcasting and streaming.
        Ac4 = AV_CODEC_ID_AC4 as i32,

        /// OSQ codec.
        /// A proprietary audio format.
        Osq = AV_CODEC_ID_OSQ as i32,

        /// QOA codec.
        /// Quite OK Audio, a simple and efficient lossy audio codec.
        Qoa = AV_CODEC_ID_QOA as i32,

        /// LC3 codec.
        /// Low Complexity Communication Codec, used in Bluetooth LE Audio.
        Lc3 = AV_CODEC_ID_LC3 as i32,

        /// DVD Subtitle codec.
        /// Subtitle format used in DVDs.
        DvdSubtitle = AV_CODEC_ID_DVD_SUBTITLE as i32,

        /// DVB Subtitle codec.
        /// Subtitle format used in DVB broadcasts.
        DvbSubtitle = AV_CODEC_ID_DVB_SUBTITLE as i32,

        /// Text codec.
        /// A simple text-based subtitle format.
        Text = AV_CODEC_ID_TEXT as i32,

        /// XSUB codec.
        /// Subtitle format used in DivX video files.
        Xsub = AV_CODEC_ID_XSUB as i32,

        /// SSA codec.
        /// SubStation Alpha subtitle format, used in anime fansubs.
        Ssa = AV_CODEC_ID_SSA as i32,

        /// MOV Text codec.
        /// Text-based subtitles stored in QuickTime/MOV containers.
        MovText = AV_CODEC_ID_MOV_TEXT as i32,

        /// HDMV PGS Subtitle codec.
        /// Blu-ray subtitle format using graphical images.
        HdmvPgsSubtitle = AV_CODEC_ID_HDMV_PGS_SUBTITLE as i32,

        /// DVB Teletext codec.
        /// Teletext format used in DVB broadcasts.
        DvbTeletext = AV_CODEC_ID_DVB_TELETEXT as i32,

        /// SRT codec.
        /// SubRip Subtitle format, one of the most common subtitle formats.
        Srt = AV_CODEC_ID_SRT as i32,

        /// MicroDVD codec.
        /// A simple subtitle format using timestamps.
        MicroDvd = AV_CODEC_ID_MICRODVD as i32,

        /// EIA-608 codec.
        /// Closed captioning format used in analog TV broadcasts.
        Eia608 = AV_CODEC_ID_EIA_608 as i32,

        /// JacoSub codec.
        /// A subtitle format used in older multimedia applications.
        JacoSub = AV_CODEC_ID_JACOSUB as i32,

        /// SAMI codec.
        /// Synchronized Accessible Media Interchange, a subtitle format from Microsoft.
        Sami = AV_CODEC_ID_SAMI as i32,

        /// RealText codec.
        /// Subtitle format used in RealMedia files.
        RealText = AV_CODEC_ID_REALTEXT as i32,

        /// STL codec.
        /// EBU STL subtitle format, used in broadcasting.
        Stl = AV_CODEC_ID_STL as i32,

        /// SubViewer 1 codec.
        /// A simple subtitle format similar to SRT.
        SubViewer1 = AV_CODEC_ID_SUBVIEWER1 as i32,

        /// SubViewer codec.
        /// A newer version of the SubViewer subtitle format.
        SubViewer = AV_CODEC_ID_SUBVIEWER as i32,

        /// SubRip codec.
        /// Another name for the SRT subtitle format.
        SubRip = AV_CODEC_ID_SUBRIP as i32,

        /// WebVTT codec.
        /// A subtitle format used for web video.
        WebVtt = AV_CODEC_ID_WEBVTT as i32,

        /// MPL2 codec.
        /// A simple subtitle format used in multimedia players.
        Mpl2 = AV_CODEC_ID_MPL2 as i32,

        /// VPlayer codec.
        /// A subtitle format used in older multimedia applications.
        VPlayer = AV_CODEC_ID_VPLAYER as i32,

        /// PJS codec.
        /// A simple subtitle format.
        Pjs = AV_CODEC_ID_PJS as i32,

        /// Advanced SSA codec.
        /// An improved version of SSA subtitles.
        Ass = AV_CODEC_ID_ASS as i32,

        /// HDMV Text Subtitle codec.
        /// A subtitle format used in Blu-ray movies.
        HdmvTextSubtitle = AV_CODEC_ID_HDMV_TEXT_SUBTITLE as i32,

        /// TTML codec.
        /// Timed Text Markup Language, used for subtitles and captions.
        Ttml = AV_CODEC_ID_TTML as i32,

        /// ARIB Caption codec.
        /// A subtitle format used in Japanese digital broadcasting.
        AribCaption = AV_CODEC_ID_ARIB_CAPTION as i32,

        /// TrueType Font codec.
        /// Used to embed font data in multimedia files.
        Ttf = AV_CODEC_ID_TTF as i32,

        /// SCTE-35 codec.
        /// Standard for inserting cue points in digital broadcasting.
        Scte35 = AV_CODEC_ID_SCTE_35 as i32,

        /// EPG codec.
        /// Electronic Program Guide data for digital TV.
        Epg = AV_CODEC_ID_EPG as i32,

        /// Binary Text codec.
        /// A proprietary subtitle format.
        BinText = AV_CODEC_ID_BINTEXT as i32,

        /// XBIN codec.
        /// A text mode animation format used in DOS.
        Xbin = AV_CODEC_ID_XBIN as i32,

        /// IDF codec.
        /// A proprietary subtitle format.
        Idf = AV_CODEC_ID_IDF as i32,

        /// OpenType Font codec.
        /// Used to embed OpenType fonts in multimedia files.
        Otf = AV_CODEC_ID_OTF as i32,

        /// SMPTE KLV codec.
        /// Metadata encoding format used in broadcasting.
        SmpteKlv = AV_CODEC_ID_SMPTE_KLV as i32,

        /// DVD Navigation codec.
        /// Data format used for interactive DVD menus.
        DvdNav = AV_CODEC_ID_DVD_NAV as i32,

        /// Timed ID3 codec.
        /// Stores metadata in streaming audio formats.
        TimedId3 = AV_CODEC_ID_TIMED_ID3 as i32,

        /// Binary Data codec.
        /// Used for arbitrary binary data storage in multimedia files.
        BinData = AV_CODEC_ID_BIN_DATA as i32,

        /// SMPTE 2038 codec.
        /// A metadata format used in digital broadcasting.
        Smpte2038 = AV_CODEC_ID_SMPTE_2038 as i32,

        /// LCEVC codec.
        /// Low Complexity Enhancement Video Coding, a scalable video enhancement format.
        Lcevc = AV_CODEC_ID_LCEVC as i32,

        /// Probe codec.
        /// Used internally by FFmpeg to detect the correct codec.
        Probe = AV_CODEC_ID_PROBE as i32,

        /// MPEG-2 Transport Stream codec.
        /// A container format for digital broadcasting.
        Mpeg2Ts = AV_CODEC_ID_MPEG2TS as i32,

        /// MPEG-4 Systems codec.
        /// A container format for MPEG-4 multimedia.
        Mpeg4Systems = AV_CODEC_ID_MPEG4SYSTEMS as i32,

        /// FFmpeg Metadata codec.
        /// Stores metadata in multimedia files.
        FfMetadata = AV_CODEC_ID_FFMETADATA as i32,

        /// Wrapped AVFrame codec.
        /// Used internally by FFmpeg to wrap raw frame data.
        WrappedAvFrame = AV_CODEC_ID_WRAPPED_AVFRAME as i32,

        /// Null Video codec.
        /// A placeholder for discarded video streams.
        VNull = AV_CODEC_ID_VNULL as i32,

        /// Null Audio codec.
        /// A placeholder for discarded audio streams.
        ANull = AV_CODEC_ID_ANULL as i32,
    }
}

impl PartialEq<i32> for AVCodecID {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<crate::ffi::AVCodecID> for AVCodecID {
    fn from(value: crate::ffi::AVCodecID) -> Self {
        AVCodecID(value as i32)
    }
}

impl From<AVCodecID> for crate::ffi::AVCodecID {
    fn from(value: AVCodecID) -> Self {
        value.0 as crate::ffi::AVCodecID
    }
}
