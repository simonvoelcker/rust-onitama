import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'
import Card from './Card'

export default class Player extends Component {

  getClasses () {
    return 'player-panel player' + this.props.playerIndex + '-panel'
  }

  render () {
    return (
      <AppConsumer>
        {({ state }) => (
          <div className={this.getClasses()}>
            { this.props.playerIndex === "0" && state.game.current_player === 0 &&
              <Card mini name='pferd' />
            }
            <div className='cards'>
              <Card name='hase' />
              <Card name='drache' />
            </div>
            { this.props.playerIndex === "1" && state.game.current_player === 1 &&
              <Card mini name='pferd' />
            }
          </div>
        )}
      </AppConsumer>
    )
  }
}

Player.contextType = AppContext
