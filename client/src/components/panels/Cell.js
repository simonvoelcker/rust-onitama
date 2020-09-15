import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Cell extends Component {
  renderPiece (piece) {
    if (piece.is_master) {
      if (piece.player === 0) {
        return <span>M</span>
      } else {
        return <span>m</span>
      }
    } else {
      if (piece.player === 0) {
        return <span>A</span>
      } else {
        return <span>a</span>
      }
    }
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='cell'>
            {this.props.piece && this.renderPiece(this.props.piece)}
          </div>
        )}
      </AppConsumer>
    )
  }
}

Cell.contextType = AppContext
