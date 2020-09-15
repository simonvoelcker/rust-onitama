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
      game: null,
    }

    this.mutations = {
      startNewGame: () => {
        return $axios.post('/games').then((response) => {
          this.setState({gameId: response.data})
          this.mutations.getGame(response.data);
        })
      },
      getGame: (gameId) => {
        return $axios.get('/games/' + gameId).then((response) => {
          this.setState({game: response.data})
          console.log(response.data)
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
