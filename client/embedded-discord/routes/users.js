var express = require('express');
const {DiscordSDK} = require("@discord/embedded-app-sdk");
var router = express.Router();

/* GET users listing. */
router.get('/', async function (req, res, next) {

    console.log("hit");
    // Instantiate the SDK
    const discordSdk = new DiscordSDK("1309255291048558632");

    console.log(`SDK instantiated ${discordSdk.toString()}`);

    async function setupDiscordSdk() {
        console.log("Setting up Discord SDK");
        await discordSdk.ready();
    }

    setupDiscordSdk().then(() => {
        console.log("Discord SDK is ready");
    }).catch((err) => {
        console.log(err);
    });

    console.log(discordSdk.sourceOrigin);

    const {code} = await discordSdk.commands.authorize({
        client_id: "1309255291048558632",
        response_type: 'code',
        state: '',
        prompt: 'none',
        // More info on scopes here: https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes
        scope: [
            'applications.commands',
            'identify',

        ],
    });

    console.log(code);

    res.send('respond with a resource');
});

module.exports = router;
