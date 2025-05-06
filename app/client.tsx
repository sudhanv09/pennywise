/// <reference types="vinxi/types/client" />
import { hydrate } from 'solid-js/web'
import { StartClient } from '@tanstack/solid-start'
import { createRouter } from './router'
import './app.css'

const router = createRouter()

hydrate(() => <StartClient router={router} />, document.body)