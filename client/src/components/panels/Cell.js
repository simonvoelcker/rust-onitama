import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Cell extends Component {

  getImageSrc () {
    let color = this.props.piece.player === 0 ? 'blue' : 'red'
    let kind = this.props.piece.is_master ? 'master' : 'apprentice'
    return 'http://localhost:3030/static/piece-' + color + '-' + kind + '.png'
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='cell'>
            {this.props.piece && <img className='piece' src={this.getImageSrc()} alt='piece' />}
          </div>
        )}
      </AppConsumer>
    )
  }
}

Cell.contextType = AppContext
