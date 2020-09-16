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
      options: null,
      selection: {
        from_position: null,
        target_position: null,
        card: null,
      }
    }

    this.mutations = {
      startNewGame: () => {
        return $axios.post('/games').then((response) => {
          this.setState({gameId: response.data})
          this.mutations.getGame(response.data);
          this.mutations.getOptions(response.data);
        })
      },
      getGame: (gameId) => {
        return $axios.get('/games/' + gameId).then((response) => {
          this.setState({game: response.data})
        })
      },
      getOptions: (gameId) => {
        return $axios.get('/games/' + gameId + '/options').then((response) => {
          this.setState({options: response.data})
        })
      },
      onCellClick: (piece, x, y) => {
        if (piece && piece.player === this.state.game.current_player) {
          // own piece clicked -> from_position selected
          this.setState({selection: {from_position: {x: x, y: y}, target_position: null, card: null}})
        } else {
          // either opponent piece clicked or no piece at all -> target_position selected
          this.setState({selection: {from_position: null, target_position: {x: x, y: y}, card: null}})
        }
      },
      onCardClick: (cardName) => {
        this.setState({selection: {from_position: null, target_position: null, card: cardName}})
      },
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
