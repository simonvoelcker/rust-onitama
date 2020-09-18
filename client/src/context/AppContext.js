import React, { Component } from 'react'

import axios from 'axios'

const $axios = axios.create({ baseURL: 'http://localhost:3030/api/', timeout: 10000, headers: { 'Content-Type': 'application/json' }})

export const AppContext = React.createContext()

export class AppProvider extends Component {
  constructor (props) {
    super(props)

    this.options = null // options from game server

    this.state = {
      gameId: null,
      game: null,
      selection: {
        fromPosition: null,
        targetPosition: null,
        cardName: null,
      },
      selectableOptions: [], // options filtered down by selection
    }

    this.mutations = {
      startNewGame: () => {
        return $axios.post('/games').then((response) => {
          this.setState({gameId: response.data})
          this.mutations.getGame(response.data)
          this.mutations.getOptions(response.data)
        })
      },
      getGame: (gameId) => {
        return $axios.get('/games/' + gameId).then((response) => {
          this.setState({game: response.data})
        })
      },
      getOptions: (gameId) => {
        return $axios.get('/games/' + gameId + '/options').then((response) => {
          this.options = response.data
          this.clearSelection()
        })
      },
      pickOption: (optionIndex) => {
        return $axios.post('/games/' + this.state.gameId + '/options/' + optionIndex).then((response) => {
          this.mutations.getGame(this.state.gameId)
          this.mutations.getOptions(this.state.gameId)
        })
      },
      onCellClick: (piece, position) => {
        if (piece && piece.player === this.state.game.current_player) {
          // own piece clicked -> from_position selected
          this.setOrKeepSelection(position, null, null)
        } else {
          // either opponent piece clicked or no piece at all -> target_position selected
          this.setOrKeepSelection(null, position, null)
        }
      },
      onCardClick: (cardName) => {
        this.setOrKeepSelection(null, null, cardName)
      },
    }
  }

  setOrKeepSelection (fromPosition, targetPosition, cardName) {
    let selection = {
      fromPosition: fromPosition || this.state.selection.fromPosition,
      targetPosition: targetPosition || this.state.selection.targetPosition,
      cardName: cardName || this.state.selection.cardName,
    }
    this.setSelection(selection)
  }

  setSelection(selection) {
    // apply selection change only if we'll have at least one option after this
    let selectableOptions = this.filterSelectionOptions(selection)
    if (selectableOptions.length > 0) {
      this.setState({
        selection: selection,
        selectableOptions: selectableOptions,
      })

      if (selection.fromPosition !== null &&
          selection.targetPosition !== null &&
          selection.cardName !== null &&
          selectableOptions.length === 1) {
        // everything selected, apply!
        let optionIndex = this.options.indexOf(selectableOptions[0])
        this.mutations.pickOption(optionIndex)
      }
    } else {
      this.clearSelection()
    }
  }

  clearSelection () {
    this.setState({
      selection: {
        fromPosition: null,
        targetPosition: null,
        cardName: null,
      },
      selectableOptions: this.state.options,
    })
  }

  filterSelectionOptions (selection) {
    let filteredOptions = this.options.filter(option => {
      if (selection.fromPosition !== null && (
          option.from_position.x !== selection.fromPosition.x ||
          option.from_position.y !== selection.fromPosition.y)) {
        return false
      }
      if (selection.targetPosition !== null && (
          option.target_position.x !== selection.targetPosition.x ||
          option.target_position.y !== selection.targetPosition.y)) {
        return false
      }
      if (selection.cardName !== null && option.card.name !== selection.cardName) {
        return false
      }
      return true
    })
    return filteredOptions
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
