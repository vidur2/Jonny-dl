const youtubedl  = require("youtube-dl-exec")
const fetch = require('node-fetch')
const fs = require("fs")
const prompt = require("prompt")

prompt.start();

prompt.get(['link', 'path'], function (err, result) {
    console.log(result.link)
    youtubedl(result.link, {
        dumpSingleJson: true,
        noWarnings: true,
        noCallHome: true,
        noCheckCertificate: true,
        preferFreeFormats: true,
        youtubeSkipDashManifest: true,
        referer: result.link
      })
        .then(output => {
            const url = output.formats[0].url;
            fetch(url).then((resp) => {
                resp.arrayBuffer().then((ab) => {
                    const buffer = Buffer.from(ab);
                    fs.createWriteStream(result.path).write(buffer)
                })
            })
        })
});
