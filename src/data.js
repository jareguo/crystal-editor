import { writable } from 'svelte/store';

// All data in this file must use writable()
// writable() gives us a reactive way to update all the elements
// that depends on it.

// Projects
export const projects = writable([]);

// Project Info (When a project is loaded)
export const project_name = writable();
export const project_path = writable();
