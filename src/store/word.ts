import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api";
import { message } from 'ant-design-vue';
import { getCustomDate } from "../utils";

export const useWordStore = defineStore('word', {
    state: () => {
        interface Dialog {
            id: number;
            person: string;
            content: string;
            created_at: String;
            updated_at: String;
        }
        interface reviewWrod {
            id: number;
            word: string;
            vocabulary_id: number;
            paraphrase: string;
            last_learned_at: string;
            next_learn_at: string;
            learn_status: number;
            show_paraphrase: boolean;
        }
        return {
            // string array
            reviewWords: [] as reviewWrod[],
            // dialog array
            wordDialogs: [] as Dialog[],
            // dialog page
            page: 1,
            // dialog 
            dialog: {} as Dialog,
        }
    },
    // 计算属性
    getters: {},
    // 用于封装业务逻辑，修改sate
    actions: {
        async getReviewWords(dt: string) {
            invoke("get_review_words", { dt: dt }).then((res: any) => {
                console.log("get_review_words", res);
                this.reviewWords = res.data;
                this.reviewWords.forEach((item: any) => {
                    item.show_paraphrase = false;
                });
            });
        },
        async changeShowParaphrase(id: number) {
            this.reviewWords.forEach((item: any) => {
                if (item.id == id) {
                    item.show_paraphrase = !item.show_paraphrase;
                }
            });
        },
        async learnWord(word: any, status: number) {
            invoke("learn_word", { id: word.id, count: word.learn_count, next: word.next_learn_at, status: status }).then((res: any) => {
                console.log("learn_word", res);
                if (res.status == "Failed") {
                    message.error(res.msg);
                }
                this.getReviewWords(word.next_learn_at);
            });
        },
        async loadDialogs() {
            invoke("get_dialogs", { page: this.page, size: 20 }).then((res: any) => {
                console.log("get_dialogs", res);
                if (res.data.length > 0) {
                    // revert data
                    let tmp = this.wordDialogs;
                    this.wordDialogs = [];
                    this.wordDialogs.push(...res.data.reverse());
                    this.wordDialogs.push(...tmp);
                    this.page++;
                } else {
                    message.warning("没有更多数据了");
                }
            });
        },
        async addDialog(person: string, content: string) {
            invoke("add_new_dialog", { person: person, content: content }).then((res: any) => {
                console.log("add_dialog", res);
                if (res.status == "Success") {
                    let dialog = {
                        id: res.data.id,
                        person: person,
                        content: content,
                        created_at: getCustomDate(0),
                        updated_at: getCustomDate(0),
                    };
                    this.wordDialogs.unshift(dialog);
                } else {
                    message.warning(res.msg);
                }
            });
        },
        async addReviewWord(word: string) {
            invoke("add_review_word", { word: word }).then((res: any) => {
                console.log("add_review_word", res);
                if (res.status == "Success") {
                    message.success("添加成功");
                    this.getReviewWords(getCustomDate(0));
                } else {
                    message.warning(res.msg);
                }
            });
        },
        async queryWord(word: string) {
            this.addDialog("me", "query: " + word);
            invoke('get_vocabulary_info', { word: word }).then((res: any) => {
                console.log("get_vocabulary_info", res);
                if (res.status == "Success") {
                    this.addDialog("orion", JSON.stringify(res.data, null, 2));
                } else {
                    message.warning(res.msg);
                }
            });
        },
        async dealOrder(order: string, content: string) {
            switch (order) {
                case "/nw": // new word
                    await this.addReviewWord(content);
                    break
                case "/qw": // query word
                    await this.queryWord(content);
                    break
                default:
            }

        },
        // init用于初始化数据，比如从后端获取数据
        init() { }
    }

})