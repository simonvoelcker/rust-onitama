import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'
import Card from './Card'

export default class Player extends Component {

  getClasses () {
    return this.props.playerIndex === '0' ? 'player-panel' : 'player-panel opponent-player-panel'
  }

  render () {
    return (
      <AppConsumer>
        {({ state }) => (
          <div className={this.getClasses()}>
            { parseInt(this.props.playerIndex) === state.game.current_player
              ? <Card mini name={state.game.public_card.name} />
              : <div className='mini-card'></div>
            }
            <div className='cards'>
              <Card name={state.game.players[this.props.playerIndex].cards[0].name} />
              <Card name={state.game.players[this.props.playerIndex].cards[1].name} />
            </div>
          </div>
        )}
      </AppConsumer>
    )
  }
}

Player.contextType = AppContext
