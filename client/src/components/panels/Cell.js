import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Cell extends Component {

  getClasses () {
    let from = this.context.state.selection.fromPosition
    if (from && from.x === this.props.position.x && from.y === this.props.position.y) {
      return 'cell from-cell'
    }
    let target = this.context.state.selection.targetPosition
    if (target && target.x === this.props.position.x && target.y === this.props.position.y) {
      return 'cell target-cell'
    }
    return 'cell'
  }

  getImageSrc () {
    let color = this.props.piece.player === 0 ? 'blue' : 'red'
    let kind = this.props.piece.is_master ? 'master' : 'apprentice'
    return 'http://localhost:3030/img/piece-' + color + '-' + kind + '-front.png'
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div
            className={this.getClasses()}
            onClick={() => mutations.onCellClick(this.props.piece, this.props.position)}>
            {this.props.piece && <img className='piece' src={this.getImageSrc()} alt='piece' />}
          </div>
        )}
      </AppConsumer>
    )
  }
}

Cell.contextType = AppContext
