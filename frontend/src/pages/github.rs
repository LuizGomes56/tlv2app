use yew::prelude::*;

use crate::img::icons::github_svg;

pub fn github() -> Html {
    html! {
        <div class="flex flex-col gap-12 max-h-screen overflow-y-auto p-12">
            <h1 class="font-bold text-4xl text-white">{ "Github repository" }</h1>
            <div class="flex flex-col gap-4">
                <h3 class="flex text-lg font-semibold items-center gap-3 mb-3 text-white">
                    { "Source code and formulas" }
                </h3>
                <p class="text-slate-300">
                    { "Formulas used here are partially available in the " }
                    <span class="font-semibold text-white">{ "Formulas" }</span>
                    { " section. You can explore how it all works by visiting my Github repositories listed below. More details about the project can be found in other sections of the application." }
                </p>
                <div class="grid sm:grid-cols-2 gap-3">
                    <a class="group bg-zinc-950/60 text-blue-400 hover:text-sky-400 hover:bg-zinc-800/60 rounded-lg p-4 transition-all duration-200"
                    href="https://github.com/LuizGomes56/tutorlolv2" target="_blank">
                        <div class="flex items-center gap-3">
                            <div class="w-5 h-5 flex-shrink-0">
                                { github_svg() }
                            </div>
                            <span class="font-semibold">{ "TutorLoLv2 Calculator Server" }</span>
                        </div>
                    </a>
                    <a class="group bg-zinc-950/60 text-blue-400 hover:text-sky-400 hover:bg-zinc-800/60 rounded-lg p-4 transition-all duration-200"
                    href="https://github.com/LuizGomes56/tlv2app" target="_blank">
                        <div class="flex items-center gap-3">
                            <div class="w-5 h-5 flex-shrink-0">
                                { github_svg() }
                            </div>
                            <span class="font-semibold">{ "TutorLoLv2 TAURI & YEW" }</span>
                        </div>
                    </a>
                </div>
                <p class="text-slate-300">
                    { "All the data is retrieved directly from the following links:" }
                </p>
                <div class="overflow-x-auto flex flex-col gap-2 pb-2">
                    <a
                        class="transition-colors w-fit font-semibold text-blue-300 hover:text-sky-400 hover:bg-sky-900/30 bg-blue-900/30 px-2 py-1 rounded-md"
                        href="https://127.0.0.1:2999/liveclientdata/allgamedata"
                        target="_blank"
                    >
                        { "https://127.0.0.1:2999/liveclientdata/allgamedata" }
                    </a>
                    <a
                        class="transition-colors w-fit font-semibold text-blue-300 hover:text-sky-400 hover:bg-sky-900/30 bg-blue-900/30 px-2 py-1 rounded-md"
                        href="https://ddragon.leagueoflegends.com/cdn/15.11.1/data/en_US/champion.json"
                        target="_blank"
                    >
                        { "https://ddragon.leagueoflegends.com/cdn/15.11.1/data/en_US/champion.json" }
                    </a>
                    <a
                        class="transition-colors w-fit font-semibold text-blue-300 hover:text-sky-400 hover:bg-sky-900/30 bg-blue-900/30 px-2 py-1 rounded-md"
                        href="https://ddragon.leagueoflegends.com/cdn/15.11.1/data/en_US/item.json"
                        target="_blank"
                    >
                        { "https://ddragon.leagueoflegends.com/cdn/15.11.1/data/en_US/item.json" }
                    </a>
                    <a
                        class="transition-colors w-fit font-semibold text-blue-300 hover:text-sky-400 hover:bg-sky-900/30 bg-blue-900/30 px-2 py-1 rounded-md"
                        href="https://ddragon.leagueoflegends.com/cdn/15.11.1/data/en_US/runesReforged.json"
                        target="_blank"
                    >
                        { "https://ddragon.leagueoflegends.com/cdn/15.11.1/data/en_US/runesReforged.json" }
                    </a>
                </div>
                <div class="flex items-center gap-2 text-[#8E8F93]">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M2.166 4.999A11.954 11.954 0 0010 1.944 11.954 11.954 0 0017.834 5c.11.65.166 1.32.166 2.001 0 5.225-3.34 9.67-8 11.317C5.34 16.67 2 12.225 2 7c0-.682.057-1.35.166-2.001zm11.541 3.708a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                    </svg>
                    <span class="font-semibold">{ "No unauthorized data sources are used." }</span>
                </div>
            </div>
        </div>
    }
}
