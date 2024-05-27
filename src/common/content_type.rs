/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::error::RResult;

#[derive(Debug)]
pub(crate) struct ContentTypeAndExtension {
    map: std::collections::HashMap<String, String>,
}
impl ContentTypeAndExtension {
    pub(crate) fn take_content_type(&self, extension: &str) -> Option<String> {
        match self.map.get(extension) {
            Some(content_type) => {
                Some(content_type.clone())
            }
            None => {
                None
            }
        }
    }
    pub(crate) fn new() -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert(".*".to_string(), "application/octet-stream".to_string());
        map.insert(".tif".to_string(), "image/tiff".to_string());
        map.insert(".001".to_string(), "application/x-001".to_string());
        map.insert(".301".to_string(), "application/x-301".to_string());
        map.insert(".323".to_string(), "text/h323".to_string());
        map.insert(".906".to_string(), "application/x-906".to_string());
        map.insert(".907".to_string(), "drawing/907".to_string());
        map.insert(".a11".to_string(), "application/x-a11".to_string());
        map.insert(".acp".to_string(), "audio/x-mei-aac".to_string());
        map.insert(".ai".to_string(), "application/postscript".to_string());
        map.insert(".aif".to_string(), "audio/aiff".to_string());
        map.insert(".aifc".to_string(), "audio/aiff".to_string());
        map.insert(".aiff".to_string(), "audio/aiff".to_string());
        map.insert(".anv".to_string(), "application/x-anv".to_string());
        map.insert(".asa".to_string(), "text/asa".to_string());
        map.insert(".asf".to_string(), "video/x-ms-asf".to_string());
        map.insert(".asp".to_string(), "text/asp".to_string());
        map.insert(".asx".to_string(), "video/x-ms-asf".to_string());
        map.insert(".au".to_string(), "audio/basic".to_string());
        map.insert(".avi".to_string(), "video/avi".to_string());
        map.insert(
            ".awf".to_string(),
            "application/vnd.adobe.workflow".to_string(),
        );
        map.insert(".biz".to_string(), "text/xml".to_string());
        map.insert(".bmp".to_string(), "application/x-bmp".to_string());
        map.insert(".bot".to_string(), "application/x-bot".to_string());
        map.insert(".c4t".to_string(), "application/x-c4t".to_string());
        map.insert(".c90".to_string(), "application/x-c90".to_string());
        map.insert(".cal".to_string(), "application/x-cals".to_string());
        map.insert(
            ".cat".to_string(),
            "application/vnd.ms-pki.seccat".to_string(),
        );
        map.insert(".cdf".to_string(), "application/x-netcdf".to_string());
        map.insert(".cdr".to_string(), "application/x-cdr".to_string());
        map.insert(".cel".to_string(), "application/x-cel".to_string());
        map.insert(".cer".to_string(), "application/x-x509-ca-cert".to_string());
        map.insert(".cg4".to_string(), "application/x-g4".to_string());
        map.insert(".cgm".to_string(), "application/x-cgm".to_string());
        map.insert(".cit".to_string(), "application/x-cit".to_string());
        map.insert(".class".to_string(), "java/*".to_string());
        map.insert(".cml".to_string(), "text/xml".to_string());
        map.insert(".cmp".to_string(), "application/x-cmp".to_string());
        map.insert(".cmx".to_string(), "application/x-cmx".to_string());
        map.insert(".cot".to_string(), "application/x-cot".to_string());
        map.insert(".crl".to_string(), "application/pkix-crl".to_string());
        map.insert(".crt".to_string(), "application/x-x509-ca-cert".to_string());
        map.insert(".csi".to_string(), "application/x-csi".to_string());
        map.insert(".css".to_string(), "text/css".to_string());
        map.insert(".cut".to_string(), "application/x-cut".to_string());
        map.insert(".dbf".to_string(), "application/x-dbf".to_string());
        map.insert(".dbm".to_string(), "application/x-dbm".to_string());
        map.insert(".dbx".to_string(), "application/x-dbx".to_string());
        map.insert(".dcd".to_string(), "text/xml".to_string());
        map.insert(".dcx".to_string(), "application/x-dcx".to_string());
        map.insert(".der".to_string(), "application/x-x509-ca-cert".to_string());
        map.insert(".dgn".to_string(), "application/x-dgn".to_string());
        map.insert(".dib".to_string(), "application/x-dib".to_string());
        map.insert(".dll".to_string(), "application/x-msdownload".to_string());
        map.insert(".doc".to_string(), "application/msword".to_string());
        map.insert(".dot".to_string(), "application/msword".to_string());
        map.insert(".drw".to_string(), "application/x-drw".to_string());
        map.insert(".dtd".to_string(), "text/xml".to_string());
        map.insert(".dwf".to_string(), "Model/vnd.dwf".to_string());
        map.insert(".dwf".to_string(), "application/x-dwf".to_string());
        map.insert(".dwg".to_string(), "application/x-dwg".to_string());
        map.insert(".dxb".to_string(), "application/x-dxb".to_string());
        map.insert(".dxf".to_string(), "application/x-dxf".to_string());
        map.insert(".edn".to_string(), "application/vnd.adobe.edn".to_string());
        map.insert(".emf".to_string(), "application/x-emf".to_string());
        map.insert(".eml".to_string(), "message/rfc822".to_string());
        map.insert(".ent".to_string(), "text/xml".to_string());
        map.insert(".epi".to_string(), "application/x-epi".to_string());
        map.insert(".eps".to_string(), "application/x-ps".to_string());
        map.insert(".eps".to_string(), "application/postscript".to_string());
        map.insert(".etd".to_string(), "application/x-ebx".to_string());
        map.insert(".exe".to_string(), "application/x-msdownload".to_string());
        map.insert(".fax".to_string(), "image/fax".to_string());
        map.insert(".fdf".to_string(), "application/vnd.fdf".to_string());
        map.insert(".fif".to_string(), "application/fractals".to_string());
        map.insert(".fo".to_string(), "text/xml".to_string());
        map.insert(".frm".to_string(), "application/x-frm".to_string());
        map.insert(".g4".to_string(), "application/x-g4".to_string());
        map.insert(".gbr".to_string(), "application/x-gbr".to_string());
        map.insert(".".to_string(), "application/x-".to_string());
        map.insert(".gif".to_string(), "image/gif".to_string());
        map.insert(".gl2".to_string(), "application/x-gl2".to_string());
        map.insert(".gp4".to_string(), "application/x-gp4".to_string());
        map.insert(".hgl".to_string(), "application/x-hgl".to_string());
        map.insert(".hmr".to_string(), "application/x-hmr".to_string());
        map.insert(".hpg".to_string(), "application/x-hpgl".to_string());
        map.insert(".hpl".to_string(), "application/x-hpl".to_string());
        map.insert(".hqx".to_string(), "application/mac-binhex40".to_string());
        map.insert(".hrf".to_string(), "application/x-hrf".to_string());
        map.insert(".hta".to_string(), "application/hta".to_string());
        map.insert(".htc".to_string(), "text/x-component".to_string());
        map.insert(".htm".to_string(), "text/html".to_string());
        map.insert(".html".to_string(), "text/html".to_string());
        map.insert(".htt".to_string(), "text/webviewhtml".to_string());
        map.insert(".htx".to_string(), "text/html".to_string());
        map.insert(".icb".to_string(), "application/x-icb".to_string());
        map.insert(".ico".to_string(), "image/x-icon".to_string());
        map.insert(".ico".to_string(), "application/x-ico".to_string());
        map.insert(".iff".to_string(), "application/x-iff".to_string());
        map.insert(".ig4".to_string(), "application/x-g4".to_string());
        map.insert(".igs".to_string(), "application/x-igs".to_string());
        map.insert(".iii".to_string(), "application/x-iphone".to_string());
        map.insert(".img".to_string(), "application/x-img".to_string());
        map.insert(
            ".ins".to_string(),
            "application/x-internet-signup".to_string(),
        );
        map.insert(
            ".isp".to_string(),
            "application/x-internet-signup".to_string(),
        );
        map.insert(".IVF".to_string(), "video/x-ivf".to_string());
        map.insert(".java".to_string(), "java/*".to_string());
        map.insert(".jfif".to_string(), "image/jpeg".to_string());
        map.insert(".jpe".to_string(), "image/jpeg".to_string());
        map.insert(".jpe".to_string(), "application/x-jpe".to_string());
        map.insert(".jpeg".to_string(), "image/jpeg".to_string());
        map.insert(".jpg".to_string(), "image/jpeg".to_string());
        map.insert(".jpg".to_string(), "application/x-jpg".to_string());
        map.insert(".js".to_string(), "application/javascript".to_string());
        map.insert(".jsp".to_string(), "text/html".to_string());
        map.insert(".la1".to_string(), "audio/x-liquid-file".to_string());
        map.insert(".lar".to_string(), "application/x-laplayer-reg".to_string());
        map.insert(".latex".to_string(), "application/x-latex".to_string());
        map.insert(".lavs".to_string(), "audio/x-liquid-secure".to_string());
        map.insert(".lbm".to_string(), "application/x-lbm".to_string());
        map.insert(".lmsff".to_string(), "audio/x-la-lms".to_string());
        map.insert(".ls".to_string(), "application/x-javascript".to_string());
        map.insert(".ltr".to_string(), "application/x-ltr".to_string());
        map.insert(".m1v".to_string(), "video/x-mpeg".to_string());
        map.insert(".m2v".to_string(), "video/x-mpeg".to_string());
        map.insert(".m3u".to_string(), "audio/mpegurl".to_string());
        map.insert(".m4e".to_string(), "video/mpeg4".to_string());
        map.insert(".mac".to_string(), "application/x-mac".to_string());
        map.insert(".man".to_string(), "application/x-troff-man".to_string());
        map.insert(".math".to_string(), "text/xml".to_string());
        map.insert(".mdb".to_string(), "application/msaccess".to_string());
        map.insert(".mdb".to_string(), "application/x-mdb".to_string());
        map.insert(
            ".mfp".to_string(),
            "application/x-shockwave-flash".to_string(),
        );
        map.insert(".mht".to_string(), "message/rfc822".to_string());
        map.insert(".mhtml".to_string(), "message/rfc822".to_string());
        map.insert(".mi".to_string(), "application/x-mi".to_string());
        map.insert(".mid".to_string(), "audio/mid".to_string());
        map.insert(".midi".to_string(), "audio/mid".to_string());
        map.insert(".mil".to_string(), "application/x-mil".to_string());
        map.insert(".mml".to_string(), "text/xml".to_string());
        map.insert(".mnd".to_string(), "audio/x-musicnet-download".to_string());
        map.insert(".mns".to_string(), "audio/x-musicnet-stream".to_string());
        map.insert(".mocha".to_string(), "application/x-javascript".to_string());
        map.insert(".movie".to_string(), "video/x-sgi-movie".to_string());
        map.insert(".mp1".to_string(), "audio/mp1".to_string());
        map.insert(".mp2".to_string(), "audio/mp2".to_string());
        map.insert(".mp2v".to_string(), "video/mpeg".to_string());
        map.insert(".mp3".to_string(), "audio/mp3".to_string());
        map.insert(".mp4".to_string(), "video/mpeg4".to_string());
        map.insert(".mpa".to_string(), "video/x-mpg".to_string());
        map.insert(".mpd".to_string(), "application/vnd.ms-project".to_string());
        map.insert(".mpe".to_string(), "video/x-mpeg".to_string());
        map.insert(".mpeg".to_string(), "video/mpg".to_string());
        map.insert(".mpg".to_string(), "video/mpg".to_string());
        map.insert(".mpga".to_string(), "audio/rn-mpeg".to_string());
        map.insert(".mpp".to_string(), "application/vnd.ms-project".to_string());
        map.insert(".mps".to_string(), "video/x-mpeg".to_string());
        map.insert(".mpt".to_string(), "application/vnd.ms-project".to_string());
        map.insert(".mpv".to_string(), "video/mpg".to_string());
        map.insert(".mpv2".to_string(), "video/mpeg".to_string());
        map.insert(".mpw".to_string(), "application/vnd.ms-project".to_string());
        map.insert(".mpx".to_string(), "application/vnd.ms-project".to_string());
        map.insert(".mtx".to_string(), "text/xml".to_string());
        map.insert(".mxp".to_string(), "application/x-mmxp".to_string());
        map.insert(".net".to_string(), "image/pnetvue".to_string());
        map.insert(".nrf".to_string(), "application/x-nrf".to_string());
        map.insert(".nws".to_string(), "message/rfc822".to_string());
        map.insert(".odc".to_string(), "text/x-ms-odc".to_string());
        map.insert(".out".to_string(), "application/x-out".to_string());
        map.insert(".p10".to_string(), "application/pkcs10".to_string());
        map.insert(".p12".to_string(), "application/x-pkcs12".to_string());
        map.insert(
            ".p7b".to_string(),
            "application/x-pkcs7-certificates".to_string(),
        );
        map.insert(".p7c".to_string(), "application/pkcs7-mime".to_string());
        map.insert(".p7m".to_string(), "application/pkcs7-mime".to_string());
        map.insert(
            ".p7r".to_string(),
            "application/x-pkcs7-certreqresp".to_string(),
        );
        map.insert(
            ".p7s".to_string(),
            "application/pkcs7-signature".to_string(),
        );
        map.insert(".pc5".to_string(), "application/x-pc5".to_string());
        map.insert(".pci".to_string(), "application/x-pci".to_string());
        map.insert(".pcl".to_string(), "application/x-pcl".to_string());
        map.insert(".pcx".to_string(), "application/x-pcx".to_string());
        map.insert(".pdf".to_string(), "application/pdf".to_string());
        map.insert(".pdf".to_string(), "application/pdf".to_string());
        map.insert(".pdx".to_string(), "application/vnd.adobe.pdx".to_string());
        map.insert(".pfx".to_string(), "application/x-pkcs12".to_string());
        map.insert(".pgl".to_string(), "application/x-pgl".to_string());
        map.insert(".pic".to_string(), "application/x-pic".to_string());
        map.insert(".pko".to_string(), "application/vnd.ms-pki.pko".to_string());
        map.insert(".pl".to_string(), "application/x-perl".to_string());
        map.insert(".plg".to_string(), "text/html".to_string());
        map.insert(".pls".to_string(), "audio/scpls".to_string());
        map.insert(".plt".to_string(), "application/x-plt".to_string());
        map.insert(".png".to_string(), "image/png".to_string());
        map.insert(".png".to_string(), "application/x-png".to_string());
        map.insert(
            ".pot".to_string(),
            "application/vnd.ms-powerpoint".to_string(),
        );
        map.insert(
            ".ppa".to_string(),
            "application/vnd.ms-powerpoint".to_string(),
        );
        map.insert(".ppm".to_string(), "application/x-ppm".to_string());
        map.insert(
            ".pps".to_string(),
            "application/vnd.ms-powerpoint".to_string(),
        );
        map.insert(
            ".ppt".to_string(),
            "application/vnd.ms-powerpoint".to_string(),
        );
        map.insert(".ppt".to_string(), "application/x-ppt".to_string());
        map.insert(".pr".to_string(), "application/x-pr".to_string());
        map.insert(".prf".to_string(), "application/pics-rules".to_string());
        map.insert(".prn".to_string(), "application/x-prn".to_string());
        map.insert(".prt".to_string(), "application/x-prt".to_string());
        map.insert(".ps".to_string(), "application/x-ps".to_string());
        map.insert(".ps".to_string(), "application/postscript".to_string());
        map.insert(".ptn".to_string(), "application/x-ptn".to_string());
        map.insert(
            ".pwz".to_string(),
            "application/vnd.ms-powerpoint".to_string(),
        );
        map.insert(".r3t".to_string(), "text/vnd.rn-realtext3d".to_string());
        map.insert(".ra".to_string(), "audio/vnd.rn-realaudio".to_string());
        map.insert(".ram".to_string(), "audio/x-pn-realaudio".to_string());
        map.insert(".ras".to_string(), "application/x-ras".to_string());
        map.insert(".rat".to_string(), "application/rat-file".to_string());
        map.insert(".rdf".to_string(), "text/xml".to_string());
        map.insert(
            ".rec".to_string(),
            "application/vnd.rn-recording".to_string(),
        );
        map.insert(".red".to_string(), "application/x-red".to_string());
        map.insert(".rgb".to_string(), "application/x-rgb".to_string());
        map.insert(
            ".rjs".to_string(),
            "application/vnd.rn-realsystem-rjs".to_string(),
        );
        map.insert(
            ".rjt".to_string(),
            "application/vnd.rn-realsystem-rjt".to_string(),
        );
        map.insert(".rlc".to_string(), "application/x-rlc".to_string());
        map.insert(".rle".to_string(), "application/x-rle".to_string());
        map.insert(
            ".rm".to_string(),
            "application/vnd.rn-realmedia".to_string(),
        );
        map.insert(".rmf".to_string(), "application/vnd.adobe.rmf".to_string());
        map.insert(".rmi".to_string(), "audio/mid".to_string());
        map.insert(
            ".rmj".to_string(),
            "application/vnd.rn-realsystem-rmj".to_string(),
        );
        map.insert(".rmm".to_string(), "audio/x-pn-realaudio".to_string());
        map.insert(
            ".rmp".to_string(),
            "application/vnd.rn-rn_music_package".to_string(),
        );
        map.insert(
            ".rms".to_string(),
            "application/vnd.rn-realmedia-secure".to_string(),
        );
        map.insert(
            ".rmvb".to_string(),
            "application/vnd.rn-realmedia-vbr".to_string(),
        );
        map.insert(
            ".rmx".to_string(),
            "application/vnd.rn-realsystem-rmx".to_string(),
        );
        map.insert(
            ".rnx".to_string(),
            "application/vnd.rn-realplayer".to_string(),
        );
        map.insert(".rp".to_string(), "image/vnd.rn-realpix".to_string());
        map.insert(
            ".rpm".to_string(),
            "audio/x-pn-realaudio-plugin".to_string(),
        );
        map.insert(".rsml".to_string(), "application/vnd.rn-rsml".to_string());
        map.insert(".rt".to_string(), "text/vnd.rn-realtext".to_string());
        map.insert(".rtf".to_string(), "application/msword".to_string());
        map.insert(".rtf".to_string(), "application/x-rtf".to_string());
        map.insert(".rv".to_string(), "video/vnd.rn-realvideo".to_string());
        map.insert(".sam".to_string(), "application/x-sam".to_string());
        map.insert(".sat".to_string(), "application/x-sat".to_string());
        map.insert(".sdp".to_string(), "application/sdp".to_string());
        map.insert(".sdw".to_string(), "application/x-sdw".to_string());
        map.insert(".sit".to_string(), "application/x-stuffit".to_string());
        map.insert(".slb".to_string(), "application/x-slb".to_string());
        map.insert(".sld".to_string(), "application/x-sld".to_string());
        map.insert(".slk".to_string(), "drawing/x-slk".to_string());
        map.insert(".smi".to_string(), "application/smil".to_string());
        map.insert(".smil".to_string(), "application/smil".to_string());
        map.insert(".smk".to_string(), "application/x-smk".to_string());
        map.insert(".snd".to_string(), "audio/basic".to_string());
        map.insert(".sol".to_string(), "text/plain".to_string());
        map.insert(".sor".to_string(), "text/plain".to_string());
        map.insert(
            ".spc".to_string(),
            "application/x-pkcs7-certificates".to_string(),
        );
        map.insert(".spl".to_string(), "application/futuresplash".to_string());
        map.insert(".spp".to_string(), "text/xml".to_string());
        map.insert(".ssm".to_string(), "application/streamingmedia".to_string());
        map.insert(
            ".sst".to_string(),
            "application/vnd.ms-pki.certstore".to_string(),
        );
        map.insert(".stl".to_string(), "application/vnd.ms-pki.stl".to_string());
        map.insert(".stm".to_string(), "text/html".to_string());
        map.insert(".sty".to_string(), "application/x-sty".to_string());
        map.insert(".svg".to_string(), "image/svg+xml".to_string());//"text/xml".to_string());
        map.insert(
            ".swf".to_string(),
            "application/x-shockwave-flash".to_string(),
        );
        map.insert(".tdf".to_string(), "application/x-tdf".to_string());
        map.insert(".tg4".to_string(), "application/x-tg4".to_string());
        map.insert(".tga".to_string(), "application/x-tga".to_string());
        map.insert(".tif".to_string(), "image/tiff".to_string());
        map.insert(".tif".to_string(), "application/x-tif".to_string());
        map.insert(".tiff".to_string(), "image/tiff".to_string());
        map.insert(".tld".to_string(), "text/xml".to_string());
        map.insert(".top".to_string(), "drawing/x-top".to_string());
        map.insert(
            ".torrent".to_string(),
            "application/x-bittorrent".to_string(),
        );
        map.insert(".tsd".to_string(), "text/xml".to_string());
        map.insert(".txt".to_string(), "text/plain".to_string());
        map.insert(".uin".to_string(), "application/x-icq".to_string());
        map.insert(".uls".to_string(), "text/iuls".to_string());
        map.insert(".vcf".to_string(), "text/x-vcard".to_string());
        map.insert(".vda".to_string(), "application/x-vda".to_string());
        map.insert(".vdx".to_string(), "application/vnd.visio".to_string());
        map.insert(".vml".to_string(), "text/xml".to_string());
        map.insert(".vpg".to_string(), "application/x-vpeg005".to_string());
        map.insert(".vsd".to_string(), "application/vnd.visio".to_string());
        map.insert(".vsd".to_string(), "application/x-vsd".to_string());
        map.insert(".vss".to_string(), "application/vnd.visio".to_string());
        map.insert(".vst".to_string(), "application/vnd.visio".to_string());
        map.insert(".vst".to_string(), "application/x-vst".to_string());
        map.insert(".vsw".to_string(), "application/vnd.visio".to_string());
        map.insert(".vsx".to_string(), "application/vnd.visio".to_string());
        map.insert(".vtx".to_string(), "application/vnd.visio".to_string());
        map.insert(".vxml".to_string(), "text/xml".to_string());
        map.insert(".wav".to_string(), "audio/wav".to_string());
        map.insert(".wax".to_string(), "audio/x-ms-wax".to_string());
        map.insert(".wb1".to_string(), "application/x-wb1".to_string());
        map.insert(".wb2".to_string(), "application/x-wb2".to_string());
        map.insert(".wb3".to_string(), "application/x-wb3".to_string());
        map.insert(".wbmp".to_string(), "image/vnd.wap.wbmp".to_string());
        map.insert(".wiz".to_string(), "application/msword".to_string());
        map.insert(".wk3".to_string(), "application/x-wk3".to_string());
        map.insert(".wk4".to_string(), "application/x-wk4".to_string());
        map.insert(".wkq".to_string(), "application/x-wkq".to_string());
        map.insert(".wks".to_string(), "application/x-wks".to_string());
        map.insert(".wm".to_string(), "video/x-ms-wm".to_string());
        map.insert(".wma".to_string(), "audio/x-ms-wma".to_string());
        map.insert(".wmd".to_string(), "application/x-ms-wmd".to_string());
        map.insert(".wmf".to_string(), "application/x-wmf".to_string());
        map.insert(".wml".to_string(), "text/vnd.wap.wml".to_string());
        map.insert(".wmv".to_string(), "video/x-ms-wmv".to_string());
        map.insert(".wmx".to_string(), "video/x-ms-wmx".to_string());
        map.insert(".wmz".to_string(), "application/x-ms-wmz".to_string());
        map.insert(".wp6".to_string(), "application/x-wp6".to_string());
        map.insert(".wpd".to_string(), "application/x-wpd".to_string());
        map.insert(".wpg".to_string(), "application/x-wpg".to_string());
        map.insert(".wpl".to_string(), "application/vnd.ms-wpl".to_string());
        map.insert(".wq1".to_string(), "application/x-wq1".to_string());
        map.insert(".wr1".to_string(), "application/x-wr1".to_string());
        map.insert(".wri".to_string(), "application/x-wri".to_string());
        map.insert(".wrk".to_string(), "application/x-wrk".to_string());
        map.insert(".ws".to_string(), "application/x-ws".to_string());
        map.insert(".ws2".to_string(), "application/x-ws".to_string());
        map.insert(".wsc".to_string(), "text/scriptlet".to_string());
        map.insert(".wsdl".to_string(), "text/xml".to_string());
        map.insert(".wvx".to_string(), "video/x-ms-wvx".to_string());
        map.insert(".xdp".to_string(), "application/vnd.adobe.xdp".to_string());
        map.insert(".xdr".to_string(), "text/xml".to_string());
        map.insert(".xfd".to_string(), "application/vnd.adobe.xfd".to_string());
        map.insert(
            ".xfdf".to_string(),
            "application/vnd.adobe.xfdf".to_string(),
        );
        map.insert(".xhtml".to_string(), "text/html".to_string());
        map.insert(".xls".to_string(), "application/vnd.ms-excel".to_string());
        map.insert(".xls".to_string(), "application/x-xls".to_string());
        map.insert(".xlw".to_string(), "application/x-xlw".to_string());
        map.insert(".xml".to_string(), "text/xml".to_string());
        map.insert(".xpl".to_string(), "audio/scpls".to_string());
        map.insert(".xq".to_string(), "text/xml".to_string());
        map.insert(".xql".to_string(), "text/xml".to_string());
        map.insert(".xquery".to_string(), "text/xml".to_string());
        map.insert(".xsd".to_string(), "text/xml".to_string());
        map.insert(".xsl".to_string(), "text/xml".to_string());
        map.insert(".xslt".to_string(), "text/xml".to_string());
        map.insert(".xwd".to_string(), "application/x-xwd".to_string());
        map.insert(".x_b".to_string(), "application/x-x_b".to_string());
        map.insert(
            ".sis".to_string(),
            "application/vnd.symbian.install".to_string(),
        );
        map.insert(
            ".sisx".to_string(),
            "application/vnd.symbian.install".to_string(),
        );
        map.insert(".x_t".to_string(), "application/x-x_t".to_string());
        map.insert(".ipa".to_string(), "application/vnd.iphone".to_string());
        map.insert(
            ".apk".to_string(),
            "application/vnd.android.package-archive".to_string(),
        );
        map.insert(
            ".xap".to_string(),
            "application/x-silverlight-app".to_string(),
        );
        Self { map }
    }
}
