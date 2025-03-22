import {DiscordSDK} from "@discord/embedded-app-sdk";
import * as process from "node:process";
import {SdkConfiguration} from "@discord/embedded-app-sdk/output/interface";


module.exports = (async function () {

    let config: SdkConfiguration = {disableConsoleLogOverride: true};
    let discordSDK = new DiscordSDK(process.env.DISCORD_SERVER_ID, config);

    await discordSDK.ready();

    console.log(discordSDK);

    const {code} = await discordSDK.commands.authorize({
        client_id: process.env.DISCORD_CLIENT_ID,
        response_type: 'code',
        state: '',
        prompt: 'none',
        scope: [
            'applications.commands',
        ],
    });

    const response = await fetch('/.proxy/api/token', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            code,
        }),
    });
    const {access_token} = await response.json();
    console.log(access_token);

})();