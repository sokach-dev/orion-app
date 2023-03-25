import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api";

export const useWordStore = defineStore('word', {
    state: () => {
        interface Dialog {
            id: number;
            person: string;
            content: string;
            date: Date;
            uuid: string;
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
                if (res.status == "failed") {
                    alert(res.msg);
                }
                this.getReviewWords(word.next_learn_at);
            });
        },
        // init用于初始化数据，比如从后端获取数据
        init() {
            this.wordDialogs = [
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orio",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orio",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orio",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orio",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
                {
                    id: 1,
                    person: "orion",
                    content: "hello",
                    date: new Date(),
                    uuid: "123",
                },
            ]
        }
    }

})