
import { writable } from 'svelte/store';

import { bind } from "svelte-simple-modal";
import Popup from 'src/components/modal/content.svelte'

export const modal = writable(null);

export interface ModalProps {
    title: string;
    message: string;
    type: 'error' | 'info'
}
// eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-argument
export const showModal =  ({ title, message, type}: ModalProps) => ():void => modal.set(bind((Popup as any), { title, message, type}))