import { defineStore } from 'pinia';

export const useWordStore = defineStore('word', {
    state: () => {
        interface Dialog {
            id: number;
            person: string;
            content: string;
            date: Date;
            uuid: string;
        }
        return {
            // string array
            reviewWords: [] as string[],
            // dialog array
            wordDialogs: [] as Dialog[],
        }
    },
    // 计算属性
    getters: {},
    // 用于封装业务逻辑，修改sate
    actions: {
        // init用于初始化数据，比如从后端获取数据
        init() {
            this.$state.reviewWords = [
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "hello", "world", "hel", "wor", "he",
                "world"];
            this.$state.wordDialogs = [
                {
                    id: 1,
                    person: "me",
                    content: "hello",
                    date: new Date("2021-01-01 00:00:00"),
                    uuid: "1"
                },
                {
                    id: 2,
                    person: "orion",
                    content: "hi",
                    date: new Date("2021-01-01 03:00:00"),
                    uuid: "1"
                },
                {
                    id: 3,
                    person: "me",
                    content: "nice to meet you",
                    date: new Date("2021-01-01 04:00:00"),
                    uuid: "1"
                },
                {
                    id: 4,
                    person: "orion",
                    content: "nice to meet you too",
                    date: new Date("2021-01-01 07:00:00"),
                    uuid: "1"
                }
            ]
        }
    }

})