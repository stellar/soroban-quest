import {DiscordSDK} from "@discord/embedded-app-sdk";
import * as process from "node:process";
import {SdkConfiguration} from "@discord/embedded-app-sdk/output/interface";


module.exports = (async function () {

    let config: SdkConfiguration = {disableConsoleLogOverride: true};
    let discordSDK = new DiscordSDK(process.env.DISCORD_CLIENT_ID, config);

    await discordSDK.ready();

    console.log(discordSDK);
})();