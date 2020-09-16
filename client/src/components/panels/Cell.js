import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Cell extends Component {

  getClasses () {
    let from_selected = this.context.state.selection.from_position
    if (from_selected && from_selected.x === this.props.x && from_selected.y === this.props.y) {
      return 'cell from-cell'
    }
    let target_selected = this.context.state.selection.target_position
    if (target_selected && target_selected.x === this.props.x && target_selected.y === this.props.y) {
      return 'cell target-cell'
    }
    return 'cell'
  }

  getImageSrc () {
    let color = this.props.piece.player === 0 ? 'blue' : 'red'
    let kind = this.props.piece.is_master ? 'master' : 'apprentice'
    return 'http://localhost:3030/static/piece-' + color + '-' + kind + '.png'
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div
            className={this.getClasses()}
            onClick={() => mutations.onCellClick(this.props.piece, this.props.x, this.props.y)}>
            {this.props.piece && <img className='piece' src={this.getImageSrc()} alt='piece' />}
          </div>
        )}
      </AppConsumer>
    )
  }
}

Cell.contextType = AppContext
