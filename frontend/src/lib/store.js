import { writable } from "svelte/store";

export const STORE_classes = writable([]);
export const STORE_teachers = writable([]);
export const STORE_subjects = writable([]);
export const STORE_rooms = writable([]);
export const STORE_relations = writable([]);

export const STORE_timetable = writable([]);

export const socket = new WebSocket("ws://127.0.0.1:3012");