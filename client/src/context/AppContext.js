import React, { Component } from 'react'

import axios from 'axios'

const $axios = axios.create({
  baseURL: process.env.NODE_ENV === 'production' ? '/api/' : 'http://localhost:3030/api/',
  timeout: 10000,
  headers: { 'Content-Type': 'application/json' }
})

export const AppContext = React.createContext()

export class AppProvider extends Component {
  constructor (props) {
    super(props)

    this.state = {
      gameId: null,
    }

    this.mutations = {
      startNewGame: () => {
        return $axios.post('/games').then((response) => {
          console.log(response)
        })
      }
    }
  }

  render () {
    return (
      <AppContext.Provider value={{ state: this.state, mutations: this.mutations }}>
        {this.props.children}
      </AppContext.Provider>
    )
  }
}

export const AppConsumer = AppContext.Consumer
